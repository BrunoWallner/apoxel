mod generation;
mod init;

use protocol::prelude::*;
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
    PushLoadChunks {
        load_coords: Vec<Coord>,
        token: Token,
        load_sender: Sender<Chunk>,
    },
    SetUpdateChunks {
        update_coords: Vec<Coord>,
        token: Token,
        update_sender: Sender<ChunkDelta>,
    },
    RequestUnloadChunk {
        coords: Vec<Coord>,
        token: Token,
    },
    PushSuperChunk {
        super_chunk: SuperChunk,
        token: Token,
        important: bool,
    },
    PlaceStructure {
        coord: Coord,
        structure: Structure,
        token: Token,
    }
}

// cloneable remote to chunkthread
#[derive(Debug, Clone)]
pub struct ChunkHandle {
    sender: Sender<Instruction>,
}
impl ChunkHandle {
    pub fn init() -> Self {
        let (tx, rx) = channel(None);
        let handle = Self { sender: tx };
        init::init(rx, handle.clone());
        handle
    }

    pub fn push_load_chunks(
        &self,
        load_coords: Vec<Coord>,
        load_sender: Sender<Chunk>,
        token: Token,
    ) {
        let _ = self.sender.send(Instruction::PushLoadChunks {
            load_coords,
            load_sender,
            token,
        }, false);
    }

    pub fn set_update_chunks(
        &self,
        update_coords: Vec<Coord>,
        update_sender: Sender<ChunkDelta>,
        token: Token,
    ) {
        let _ = self.sender.send(Instruction::SetUpdateChunks {
            update_coords,
            update_sender,
            token,
        }, false);
    }

    pub fn unload_chunks(&self, coords: Vec<Coord>, token: Token) {
        let _ = self
            .sender
            .send(Instruction::RequestUnloadChunk { coords, token }, false)
            .unwrap();
    }

    pub fn place_structure(&self, coord: Coord, structure: Structure, token: Token) {
        let _ = self
            .sender
            .send(Instruction::PlaceStructure { coord, structure, token }, true)
            .unwrap();
    }

    pub(crate) fn push_super_chunk(&self, super_chunk: SuperChunk, token: Token, important: bool) {
        let _ = self
            .sender
            .send(Instruction::PushSuperChunk { super_chunk, token, important }, false)
            .unwrap();
    }
}
