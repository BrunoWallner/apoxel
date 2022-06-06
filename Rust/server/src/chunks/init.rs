use crate::channel::*;
use super::Instruction;
use Instruction::*;
use std::thread;
use std::collections::BTreeMap;
use protocol::{Coord, chunk::Chunk};
use super::generation::generate;
use crate::CONFIG;

pub(super) fn init(
    rx: Receiver<Instruction>,
) {
    thread::spawn(move || {
        let mut chunks: BTreeMap<Coord, Chunk> = BTreeMap::default();
        let mut leftover: BTreeMap<Coord, Chunk> = BTreeMap::default();

        while let Some(instruction) = rx.recv() {
            match instruction {
                RequestChunks { coords, sender } => {
                    let mut chunk_buffer: Vec<Chunk> = Vec::with_capacity(coords.len());
                    for coord in coords.iter() {
                        if let Some(chunk) = chunks.get(coord) {
                            chunk_buffer.push(chunk.clone());
                        }
                        // generate
                        let mut super_chunk = generate(Chunk::new(*coord), CONFIG.chunks.seed);
                        // log::info!("chunk: {:#?}", super_chunk);
                        // extract main chunk
                        let mc_coord = super_chunk.main_chunk;
                        let mut main_chunk  = super_chunk.chunks.remove(&mc_coord).unwrap();
                        // log::info!("chunk: {:?}", main_chunk);
                        // merge main chunk with leftover if found
                        if let Some(left) = leftover.get(&mc_coord) {
                            main_chunk.merge(left);
                            leftover.remove(&mc_coord);
                        }
                        // push chunk to chunks and send it
                        // log::info!("chunk: {:?}", main_chunk);
                        if !main_chunk.is_empty() {
                            chunks.insert(mc_coord, main_chunk.clone());
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
                            if let Some(chunk) = chunks.get_mut(coord) {
                                chunk.merge(&final_left);
                            }
                        }
                    }
                    let _ = sender.send(chunk_buffer);
                }
                UnloadChunk { coords } => {
                    for coord in coords.iter() {
                        chunks.remove(coord);
                    }
                }
            }
        }
        log::info!("shutdown ChunkHandle");
    });
}