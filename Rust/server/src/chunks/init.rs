use crate::channel::*;
use super::Instruction;
use Instruction::*;
use std::thread;
use std::collections::BTreeMap;
use protocol::{Coord, chunk::Chunk};
use super::generation::generate;
use crate::CONFIG;
use super::StoredChunk;

pub(super) fn init(
    rx: Receiver<Instruction>,
    chunk_update_sender: Sender<Coord>,
) {
    thread::spawn(move || {
        let mut chunks: BTreeMap<Coord, StoredChunk> = BTreeMap::default();
        let mut leftover: BTreeMap<Coord, Chunk> = BTreeMap::default();

        while let Some(instruction) = rx.recv() {
            match instruction {
                // INFO! only properly works, when already in chunk register
                RequestChunks { coords, token, sender } => {
                    let mut chunk_buffer: Vec<Chunk> = Vec::new();
                    for coord in coords.iter() {
                        if let Some(chunk) = chunks.get_mut(coord) {
                            chunk.mark_needed_by(token);
                            chunk_buffer.push(chunk.chunk);
                        } else {
                            let super_chunk = generate(Chunk::new(*coord), CONFIG.chunks.seed);
        
                            // handle leftovers of generated chunk
                            for (coord, left) in super_chunk.chunks.iter() {
                                let final_left: Chunk;
                                if let Some(l) = leftover.get_mut(coord) {
                                    l.merge(left);
                                    final_left = *l;
                                } else {
                                    leftover.insert(*coord, *left);
                                    final_left = *left;
                                }
        
                                // apply final_left to chunk, when found
                                if let Some(stored_chunk) = chunks.get_mut(coord) {
                                    stored_chunk.chunk.merge(&final_left);
                                }
                            }

                            // extract main chunk
                            let main_chunk  = *leftover.get(coord).unwrap();
                            // push chunk to chunks
                            if !main_chunk.is_empty() {
                                let mut stored_chunk = StoredChunk::new(main_chunk);
                                stored_chunk.mark_needed_by(token);
                                chunks.insert(*coord, stored_chunk);
                                chunk_buffer.push(main_chunk);
                            }
                        }
                    }
                    let _ = sender.send(chunk_buffer);
                }
                RequestUnloadChunk { coords, token } => {
                    for coord in coords.iter() {
                        if let Some(chunk) = chunks.get_mut(coord) {
                            chunk.mark_unneeded_by(&token);
                            if !chunk.is_needed() {
                                chunks.remove(coord);
                            }
                        }
                    }
                }
            }
        }
        log::warn!("unexpected shutdown of ChunkHandle");
    });
}