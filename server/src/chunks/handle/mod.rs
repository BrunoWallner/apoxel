mod loader;
mod unloader;

use tokio::sync::mpsc;
use std::collections::HashMap;
use protocol::{Coord, PlayerCoord};
use protocol::chunk::Structure;
use protocol::chunk::{Chunk, SuperChunk};
use protocol::chunk::get_chunk_coords;
use crate::client::Event as ClientEvent;

use crate::player::handle::Handle as PlayerHandle;
use crate::config::CONFIG;

fn coord_converter(coords: Vec<PlayerCoord>) -> Vec<Coord> {
    let mut buf: Vec<Coord> = Vec::with_capacity(coords.len());
    for coord in coords.iter() {
        buf.push( get_chunk_coords(&[coord[0] as i64, coord[1] as i64, coord[2] as i64]).0 )
    }
    buf
}

// to broadcast chunkupdates, so it can be known if one should request
use crate::broadcast::BroadCast;

#[derive(Clone, Debug)]
pub enum Instruction {
    Load(Vec<Coord>),
    Unload(Vec<Coord>),
    FlushLoadQueue,
    RequestChunks{coords: Vec<Coord>, sender: mpsc::Sender<Vec<Chunk>>},

    // for unload detection
    RequestKeys(mpsc::Sender<Vec<Coord>>),

    // block or structure placing 
    PlaceStructure{coord: Coord, structure: Structure},

    // sent from generator
    PushChunks(HashMap<Coord, Chunk>),
    CheckIfLoaded{coords: Vec<Coord>, sender: mpsc::Sender<Vec<bool>>},

    // register to chunk-update broadcaster
    RegisterClient(mpsc::Sender<ClientEvent>),
}

#[derive(Clone, Debug)]
pub struct Handle {
    sender: mpsc::Sender<Instruction>,
}
impl Handle {
    pub fn init(player_handle: PlayerHandle) -> Self {
        let (tx, rx) = mpsc::channel(4096);

        let handle = Self {sender: tx};

        let h = handle.clone();
        tokio::spawn(async move {
            init(rx, h, player_handle).await;
        });

        handle
    }

    pub async fn load(&self, coords: Vec<Coord>) {
        self.sender.send(Instruction::Load(coords)).await.unwrap();
    }
    pub async fn unload(&self, coords: Vec<Coord>) {
        self.sender.send(Instruction::Unload(coords)).await.unwrap();
    }
    pub async fn flush_load_queue(&self) {
        self.sender.send(Instruction::FlushLoadQueue).await.unwrap();
    }
    pub async fn register_client(&self, sender: mpsc::Sender<ClientEvent>) {
        self.sender.send(Instruction::RegisterClient(sender)).await.unwrap();
    }
    pub async fn request(&self, coords: Vec<Coord>) -> Vec<Chunk> {
        let (tx, mut rx) = mpsc::channel(1);
        self.sender.send(Instruction::RequestChunks{coords, sender: tx}).await.unwrap();
        rx.recv().await.unwrap()
    }
    pub async fn get_keys(&self) -> Vec<Coord> {
        let (tx, mut rx) = mpsc::channel(1);
        self.sender.send(Instruction::RequestKeys(tx)).await.unwrap();
        rx.recv().await.unwrap()
    }
    pub async fn place_structure(&self, coord: Coord, structure: Structure) {
        self.sender.send(Instruction::PlaceStructure{coord, structure}).await.unwrap();
    }
    pub async fn push_chunks(&self, chunks: HashMap<Coord, Chunk>) {
        self.sender.send(Instruction::PushChunks(chunks)).await.unwrap();
    }
    pub async fn check_if_loaded(&self, coords: Vec<Coord>) -> Vec<bool> {
        let (tx, mut rx) = mpsc::channel(1);
        self.sender.send(Instruction::CheckIfLoaded{coords, sender: tx}).await.unwrap();
        rx.recv().await.unwrap()
    }
}

async fn init(mut receiver: mpsc::Receiver<Instruction>, handle: Handle, player_handle: PlayerHandle) {
    // loader
    let h = handle.clone();
    tokio::spawn(async move {
        loader::init_flusher(h).await;
    });

    let h = handle.clone();
    let p = player_handle.clone();
    tokio::spawn(async move {
        loader::player_chunk_loader(h, p).await;
    });

    // unloader
    let h = handle.clone();
    tokio::spawn(async move {
        unloader::init(h, player_handle).await;
    });

    let client_broadcast: BroadCast<ClientEvent> = BroadCast::init();

    let mut chunks: HashMap<Coord, Chunk> = HashMap::default();
    let mut load_queue: Vec<Coord> = Vec::new();

    loop {
        match receiver.recv().await {
            Some(i) => match i {
                Instruction::Load(coords) => {
                    for coord in coords.iter() {
                        if !load_queue.contains(coord) {
                            load_queue.push(*coord);
                        }
                    }
                }
                Instruction::Unload(coords) => {
                    for coord in coords.iter() {
                        chunks.remove(coord);
                    }
                }
                Instruction::FlushLoadQueue => {
                    let gpc = CONFIG.chunks.generations_per_cycle as usize;

                    let queue = if load_queue.len() > gpc {
                        load_queue.drain(0..gpc).as_slice().to_vec()
                    } else {
                        load_queue.drain(..).as_slice().to_vec()
                    };
                    let h = handle.clone();
                    tokio::spawn(async move {
                        loader::load(queue, h).await
                    });
                }
                Instruction::RequestChunks{coords, sender} => {
                    let mut chunk_buf: Vec<Chunk> = Vec::new();
                    for coord in coords.iter() {
                        if let Some(chunk) = chunks.get(coord) {
                            chunk_buf.push(*chunk);
                        }
                    }
                    sender.send(chunk_buf).await.unwrap();
                }
                Instruction::RequestKeys(sender) => {
                    let mut keys: Vec<[i64; 3]> = Vec::new();
                    chunks.keys().for_each(|key| {
                        keys.push(*key);
                    });
                    sender.send(keys).await.unwrap();
                }
                Instruction::PlaceStructure{coord, structure} => {
                    if let Some(chunk) = chunks.get(&coord) {
                        let mut super_chunk = SuperChunk::new((chunk.coord, *chunk)); // BAD
                        super_chunk.place_structure(&structure, coord);
                        apply_superchunk(&mut chunks, super_chunk.clone());
                        for key in super_chunk.chunks.keys() {
                            client_broadcast.send(ClientEvent::ChunkUpdate(*key)).await;
                        }
                    }
                }
                Instruction::PushChunks(c) => {
                    for (key, chunk) in c.iter() {
                        chunks.insert(*key, *chunk);
                        client_broadcast.send(ClientEvent::ChunkUpdate(*key)).await;
                    }
                }
                Instruction::CheckIfLoaded{coords, sender} => {
                    let mut buf: Vec<bool> = Vec::new();
                    for coord in coords.iter() {
                        if chunks.get(coord).is_some() {
                            buf.push(true)
                        } else {
                            buf.push(false)
                        }
                    }
                    sender.send(buf).await.unwrap();
                },
                Instruction::RegisterClient(sender) => {
                    client_broadcast.register(sender).await;
                }
            }
            None => (panic!("Chunk handle cannot receive any more instructions"))
        }
    }
}

fn apply_superchunk(chunks: &mut HashMap<Coord, Chunk>, super_chunk: SuperChunk) {
    for (key, data) in super_chunk.chunks.iter() {
        //voxels.map.insert(*key, *data);
        if let Some(voxels) = chunks.get_mut(key) {
            voxels.merge(data);
        } else {
            chunks.insert(*key, *data);
        }
    }
}