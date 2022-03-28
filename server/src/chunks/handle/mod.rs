mod loader;
mod unloader;

use tokio::sync::mpsc;
use std::collections::{HashMap, HashSet};
use protocol::{Token, Coord, PlayerCoord};
use protocol::chunk::Structure;
use protocol::chunk::{Chunk, SuperChunk};
use protocol::chunk::get_chunk_coords;

use crate::player::handle::Handle as PlayerHandle;

fn coord_converter(coords: Vec<PlayerCoord>) -> Vec<Coord> {
    let mut buf: Vec<Coord> = Vec::with_capacity(coords.len());
    for coord in coords.iter() {
        buf.push( get_chunk_coords(&[coord[0] as i64, coord[1] as i64, coord[2] as i64]).0 )
    }
    buf
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Load,
    Unload(Vec<Coord>),

    RequestChunks{chunks: Vec<Coord>, token: Token, sender: mpsc::Sender<Chunk>},

    // for unload detection
    RequestKeys(mpsc::Sender<Vec<Coord>>),

    // block or structure placing 
    PlaceStructure{coord: Coord, structure: Structure},

    // sent from generator
    PushChunks(HashMap<Coord, Chunk>),
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

    pub async fn load(&self) {
        self.sender.send(Instruction::Load).await.unwrap();
    }
    pub async fn unload(&self, coords: Vec<Coord>) {
        self.sender.send(Instruction::Unload(coords)).await.unwrap();
    }
    pub async fn request(&self, chunks: Vec<Coord>, token: Token, sender: mpsc::Sender<Chunk>) {
        self.sender.send(Instruction::RequestChunks{chunks, token, sender}).await.unwrap();
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
}

async fn init(mut receiver: mpsc::Receiver<Instruction>, handle: Handle, player_handle: PlayerHandle) {
    // load_requester
    let handle_clone = handle.clone();
    loader::init_load_requester(handle_clone);

    // unloader
    let handle_clone = handle.clone();
    tokio::spawn(async move {
        unloader::init(handle_clone, player_handle).await;
    });

    let mut chunks: HashMap<Coord, Chunk> = HashMap::default();

    // chunk load requests of clients
    let mut requests: HashMap<Token, (mpsc::Sender<Chunk>, Vec<Coord>)> = HashMap::default();
    let mut queued: HashSet<Coord> = HashSet::default();

    loop {
        match receiver.recv().await {
            Some(i) => match i {
                Instruction::Load => {
                    for (_token, (sender, coords)) in requests.iter_mut() {
                        while let Some(coord) = coords.pop() {
                            if let Some(chunk) = chunks.get(&coord) {
                                sender.send(*chunk).await.unwrap();
                            } else {
                                // start generating if not already queued
                                if !queued.contains(&coord) {
                                    queued.insert(coord);

                                    // load chunk, send it to client and then to handle
                                    let handle_clone = handle.clone();
                                    let coord = coord.clone();
                                    let sender = sender.clone();
                                    tokio::spawn(async move {
                                        let superchunk = super::generation::generate(Chunk::new(coord), 9872345);
                                        sender.send(*superchunk.chunks.get(&coord).unwrap()).await.unwrap();
                                        handle_clone.push_chunks(superchunk.chunks).await;
                                    });
                                }
                            }
                        }

                    }
                    // cleanup
                    for (token, (_sender, coords)) in requests.clone().into_iter() {
                        if coords.len() == 0 {
                            requests.remove(&token);
                        }
                    }
                }
                Instruction::PushChunks(c) => {
                    for (key, chunk) in c.iter() {
                        chunks.insert(*key, *chunk);
                        queued.remove(key);
                    }
                }
                Instruction::Unload(coords) => {
                    for coord in coords.iter() {
                        chunks.remove(coord);
                    }
                }
                Instruction::RequestChunks{chunks, token, sender} => {
                    if let Some(request) = requests.get_mut(&token) {
                        *request = (sender, chunks);
                    } else {
                        requests.insert(token, (sender, chunks));
                    }
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
                    }
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