mod init;

use crate::channel::*;
use protocol::{Coord, chunk::Chunk};

#[derive(Debug, Clone)]
enum Instruction {
    RequestChunk{coord: Coord, sender: Sender<Chunk>}
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

    pub fn request_chunk(&self, coord: Coord) -> Option<Chunk> {
        let (tx, rx) = channel();
        let _ = self.sender.send(Instruction::RequestChunk{coord, sender: tx});
        rx.recv()
    }
}
