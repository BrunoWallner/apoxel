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
                            chunk_buffer.push(chunk.chunk.clone());
                        } else {
                            // generate new
                            let mut super_chunk = generate(Chunk::new(*coord), CONFIG.chunks.seed);
                            
                            // extract main chunk and merge with leftover
                            let mut main_chunk = super_chunk.remove_main_chunk();
                            if let Some(left) = leftover.get(coord /* coord of main_chunk */) {
                                // log::info!("merged");
                                main_chunk.merge(left);
                            }
                            if !main_chunk.is_empty() {
                                // main_chunk is not allowed to be modified from now on
                                chunk_buffer.push(main_chunk.clone());
                                let mut stored_chunk = StoredChunk::new(main_chunk.clone());
                                stored_chunk.mark_needed_by(token);
                                chunks.insert(*coord, stored_chunk);
                            }

                            // handle leftovers of generated chunk
                            for (coord, left) in super_chunk.chunks.iter() {
                                // apply final_left to chunk, when found
                                if let Some(stored_chunk) = chunks.get_mut(coord) {
                                    let mut left = left.clone();
                                    if let Some(l) = leftover.remove(coord) {
                                       left.merge(&l);
                                    }
                                    stored_chunk.chunk.merge(&left);
                                    let _ = chunk_update_sender.send(*coord);
                                } else {
                                    if let Some(l) = leftover.get_mut(coord) {
                                        l.merge(left);
                                    } else {
                                        leftover.insert(*coord, left.clone());
                                    }
                                }
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