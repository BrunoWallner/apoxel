mod init;
mod generation;

use crate::channel::*;
use protocol::{Token, Coord, chunk::Chunk, chunk::SuperChunk};
use std::collections::HashSet;
use crate::users::Users;

#[derive(Debug, Clone)]
struct StoredChunk {
    pub chunk: Chunk,
    needed_by: HashSet<Token>,
}
impl StoredChunk {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            needed_by: HashSet::default(),
        }
    }
    pub fn is_needed(&self) -> bool {
        !self.needed_by.is_empty()
    }
    pub fn mark_needed_by(&mut self, token: Token) {
        self.needed_by.insert(token);
    }
    pub fn mark_unneeded_by(&mut self, token: &Token) {
        self.needed_by.remove(token);
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    RequestChunks{coords: Vec<Coord>, token: Token, sender: Sender<Vec<Chunk>>},
    RequestUnloadChunk{coords: Vec<Coord>, token: Token},
}

// cloneable remote to chunkthread
#[derive(Debug, Clone)]
pub struct ChunkHandle {
    sender: Sender<Instruction>
}
impl ChunkHandle {
    pub fn init(chunk_update_sender: Sender<Coord>) -> Self {
        let (tx, rx) = channel();
        init::init(rx, chunk_update_sender);
        Self {sender: tx}
    }

    pub fn request_chunks(&self, coords: Vec<Coord>, token: Token) -> Option<Vec<Chunk>> {
        let (tx, rx) = channel();
        let _ = self.sender.send(Instruction::RequestChunks{coords, sender: tx, token});
        rx.recv()
    }

    pub fn unload_chunks(&self, coords: Vec<Coord>, token: Token) {
        let _ = self.sender.send(Instruction::RequestUnloadChunk{coords, token});
    }
}
