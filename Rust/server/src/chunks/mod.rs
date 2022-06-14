mod init;
mod generation;

use crate::channel::*;
use protocol::{Token, Coord, chunk::Chunk, chunk::SuperChunk};
use std::collections::HashSet;

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
    RequestChunks{coords: Vec<Coord>, token: Token, sender: Sender<Chunk>},
    RequestUnloadChunk{coords: Vec<Coord>, token: Token},
    PushSuperChunk{super_chunk: SuperChunk, token: Token},
}

// cloneable remote to chunkthread
#[derive(Debug, Clone)]
pub struct ChunkHandle {
    sender: Sender<Instruction>
}
impl ChunkHandle {
    pub fn init() -> Self {
        let (tx, rx) = channel();
        let handle =         Self {sender: tx};
        init::init(rx, handle.clone());
        handle
    }

    pub fn request_chunks(&self, coords: Vec<Coord>, token: Token, sender: Sender<Chunk>) {
        let _ = self.sender.send(Instruction::RequestChunks{coords, sender, token});
    }

    pub fn unload_chunks(&self, coords: Vec<Coord>, token: Token) {
        let _ = self.sender.send(Instruction::RequestUnloadChunk{coords, token});
    }

    pub(crate) fn push_super_chunk(&self, super_chunk: SuperChunk, token: Token) {
        let _ = self.sender.send(Instruction::PushSuperChunk{super_chunk, token});
    }
}
