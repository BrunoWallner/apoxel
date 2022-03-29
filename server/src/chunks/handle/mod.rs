mod loader;
mod unloader;

use tokio::sync::mpsc;
use std::collections::{HashMap, HashSet};
use protocol::{Token, Coord, PlayerCoord};
use protocol::chunk::Structure;
use protocol::chunk::{Chunk, SuperChunk};
use protocol::chunk::get_chunk_coords;

use crate::player::handle::Handle as PlayerHandle;
use crate::config::CONFIG;

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
    PushChunks(SuperChunk),
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
    pub async fn push_chunks(&self, chunk: SuperChunk) {
        self.sender.send(Instruction::PushChunks(chunk)).await.unwrap();
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

    let mut leftover: HashMap<Coord, Chunk> = HashMap::default();
    let mut chunks: HashMap<Coord, Chunk> = HashMap::default();

    // chunk load requests of clients
    let mut requests: HashMap<Token, (mpsc::Sender<Chunk>, Vec<Coord>)> = HashMap::default();
    let mut queued: HashSet<Coord> = HashSet::default();

    loop {
        match receiver.recv().await {
            Some(i) => match i {
                Instruction::Load => {
                    let mut finished: Vec<Token> = Vec::new();
                    for (token, (sender, coords)) in requests.iter_mut() {
                        // multiple gens per cylce
                        'per_player: for _ in 0..CONFIG.chunks.generations_per_cycle {
                            if let Some(coord) = coords.pop() {
                                // when chunk found send it to requester
                                if let Some(chunk) = chunks.get_mut(&coord) {
                                    sender.send(*chunk).await.unwrap();
                                } else {
                                    // start generating if not already queued
                                    if !queued.contains(&coord) {
                                        queued.insert(coord);
    
                                        // load chunk and send it to handle
                                        let handle_clone = handle.clone();
                                        let coord = coord.clone();
                                        tokio::spawn(async move {
                                            let superchunk = super::generation::generate(Chunk::new(coord), 82765945);
                                            handle_clone.push_chunks(superchunk).await;
                                        });
                                    }
                                }
                            } else {
                                // no request left of specific token, mark for deletion
                                finished.push(*token);
                                break 'per_player;
                            }
                        }
                    }
                    // deletion
                    for token in finished.iter() {
                        requests.remove(token);
                    }
                }
                Instruction::PushChunks(mut s_c) => {
                    // extract main chunk
                    let mut main_chunk = s_c.chunks.remove(&s_c.main_chunk).unwrap();
                    queued.remove(&s_c.main_chunk);

                    // handle main chunk and merge when leftover chunk is detected
                    if let Some(left) = leftover.get(&s_c.main_chunk) {
                        main_chunk.merge(left);
                        leftover.remove(&s_c.main_chunk);
                    }
                    chunks.insert(s_c.main_chunk, main_chunk);

                    // handle leftovers
                    for (key, left_chunk) in s_c.chunks.iter() {
                        let left;
                        if let Some(leftover) = leftover.get_mut(key) {
                            leftover.merge(left_chunk);
                            left = *leftover;
                        } else {
                            leftover.insert(*key, *left_chunk);
                            left = *left_chunk;
                        }

                        // apply leftover to chunk, when found
                        if let Some(chunk) = chunks.get_mut(key) {
                            chunk.merge(&left);
                        }
                    }
                }
                Instruction::Unload(coords) => {
                    for coord in coords.iter() {
                        chunks.remove(coord);
                        leftover.remove(coord);
                    }
                }
                Instruction::RequestChunks{chunks, token, sender} => {
                    if let Some( (request_sender, request_chunks)) = requests.get_mut(&token) {
                        *request_sender = sender;
                        *request_chunks = chunks;
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
                        let mut super_chunk = SuperChunk::new(*chunk); // BAD
                        super_chunk.place_structure(&structure, coord, [false, false, false]);
                        apply_superchunk(&mut chunks, super_chunk.clone());
                    }
                }
            }
            None => (panic!("Chunk handle cannot receive any more instructions"))
        }
    }
}

// for structure placing
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