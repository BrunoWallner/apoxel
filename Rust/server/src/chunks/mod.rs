mod init;
mod generation;

use crate::channel::*;
use protocol::{Coord, chunk::Chunk};

#[derive(Debug, Clone)]
enum Instruction {
    RequestChunks{coords: Vec<Coord>, sender: Sender<Vec<Chunk>>},
    UnloadChunk{coords: Vec<Coord>},
}

// cloneable remote to chunkthread
#[derive(Debug, Clone)]
pub struct ChunkHandle {
    sender: Sender<Instruction>
}
impl ChunkHandle {
    pub fn init() -> Self {
        let (tx, rx) = channel();
        init::init(rx);
        Self {sender: tx}
    }

    pub fn request_chunks(&self, coords: Vec<Coord>) -> Option<Vec<Chunk>> {
        let (tx, rx) = channel();
        let _ = self.sender.send(Instruction::RequestChunks{coords, sender: tx});
        rx.recv()
    }

    pub fn unload_chunks(&self, coords: Vec<Coord>) {
        let _ = self.sender.send(Instruction::UnloadChunk{coords});
    }
}
