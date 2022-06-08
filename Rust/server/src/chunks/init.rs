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
) {
    thread::spawn(move || {
        let mut chunks: BTreeMap<Coord, StoredChunk> = BTreeMap::default();
        let mut leftover: BTreeMap<Coord, Chunk> = BTreeMap::default();

        while let Some(instruction) = rx.recv() {
            match instruction {
                RequestChunks { coords, sender, token } => {
                    let mut chunk_buffer: Vec<Chunk> = Vec::with_capacity(coords.len());
                    for coord in coords.iter() {
                        if let Some(chunk) = chunks.get_mut(coord) {
                            chunk.mark_needed_by(token);
                            if let Some(left) = leftover.get(&chunk.chunk.coord) {
                                chunk.chunk.merge(left);
                            }
                            chunk_buffer.push(chunk.chunk);
                        } else {
                            // WARN! this else might be wrong
                            // generate
                             let mut super_chunk = generate(Chunk::new(*coord), CONFIG.chunks.seed);
                            // extract main chunk
                            let mc_coord = super_chunk.main_chunk;
                            let mut main_chunk  = super_chunk.chunks.remove(&mc_coord).unwrap();
                            // merge main chunk with leftover if found
                            if let Some(left) = leftover.get(&mc_coord) {
                                main_chunk.merge(left);
                                leftover.remove(&mc_coord);
                            }
                            // push chunk to chunks and send it
                            if !main_chunk.is_empty() {
                                chunks.insert(mc_coord, StoredChunk::new(main_chunk.clone()));
                                chunk_buffer.push(main_chunk);
                            }
        
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
        log::info!("shutdown ChunkHandle");
    });
}