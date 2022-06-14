use crate::channel::*;
use super::Instruction;
use Instruction::*;
use std::thread;
use std::collections::{BTreeMap, HashMap, BTreeSet};
use protocol::{Coord, chunk::Chunk, Token};
use super::generation::generate;
use crate::CONFIG;
use super::StoredChunk;

// generates chunks nonblockingly
pub(super) fn init(
    rx: Receiver<Instruction>,
    chunk_handle: super::ChunkHandle,
    chunk_update_sender: Sender<Coord>,
) {
    thread::spawn(move || {
        let threadpool = threadpool::ThreadPool::new(2);

        let mut chunks: BTreeMap<Coord, StoredChunk> = BTreeMap::default();
        let mut leftover: BTreeMap<Coord, Chunk> = BTreeMap::default();

        let mut requests: Vec<(BTreeSet<Coord>, Sender<Chunk>)> = Vec::new();

        while let Some(instruction) = rx.recv() {
            match instruction {
                // INFO: main bottleneck in whole chunkhandle
                PushSuperChunk{mut super_chunk, token} => {
                    log::info!("chunks: {}, leftover: {}", chunks.len(), leftover.len());

                    /* --- PHASE 1 --- */
                    // extract main_chunk and merge it with leftover
                    let mut main_chunk = super_chunk.remove_main_chunk();
                    if let Some(left) = leftover.remove(&main_chunk.coord) {
                        main_chunk.merge(&left);
                    }
                    if !main_chunk.is_empty() {
                        // send main_chunk to all reqester if requested
                        let mut to_delete: Vec<usize> = Vec::new();
                        for (i, (coords, sender)) in requests.iter_mut().enumerate() {
                            if coords.remove(&main_chunk.coord) {
                                let _ = sender.send(main_chunk.clone());
                            }
                            // mark reqest entry as removable if empty
                            if coords.is_empty() {to_delete.push(i)}
                        }
                        // delete empty request entries
                        for (i, to_delete) in to_delete.iter().enumerate() {
                            requests.remove(to_delete - i);
                        }

                        /* --- PHASE 2 --- */
                        // push main_chunk to chunks and mark it as needed
                        let mc_coord = main_chunk.coord;
                        let mut stored_chunk = StoredChunk::new(main_chunk);
                        stored_chunk.mark_needed_by(token);
                        chunks.insert(mc_coord, stored_chunk);
                    }

                    /* --- PHASE 3 --- */
                    // handle leftovers
                    for (coord, left) in super_chunk.chunks.into_iter() {
                        if let Some(already_left) = leftover.get_mut(&coord) {
                            already_left.merge(&left);
                        } else {
                            leftover.insert(coord, left);
                        }

                        // when generated chunk is found at leftover coord merge it with leftover
                        if let Some(stored_chunk) = chunks.get_mut(&coord) {
                            // guaranteed not to panic because of above code
                            let left = leftover.remove(&coord).unwrap();
                            stored_chunk.chunk.merge(&left);
                            // let _ = chunk_update_sender.send(stored_chunk.chunk.coord);
                        }
                    }
                }
                // INFO! only properly works, when already in chunk register
                RequestChunks { coords, token, sender } => {
                    // filtering already generated coords and send them
                    let mut new_coords: BTreeSet<Coord> = BTreeSet::default();
                    for coord in coords.iter() {
                        if let Some(chunk) = chunks.get(coord) {
                            let mut chunk = chunk.clone().chunk;
                            if let Some(left) = leftover.get(coord) {
                                chunk.merge(left);
                            }
                            let _ = sender.send(chunk.clone());
                        } else {
                            new_coords.insert(*coord);
                        }
                    }
                    requests.push((new_coords.clone(), sender));

                    for coord in new_coords.into_iter() {
                        let chunk_handle = chunk_handle.clone();
                        threadpool.execute(move || {
                            let super_chunk = generate(Chunk::new(coord), CONFIG.chunks.seed);
                            if !super_chunk.get_main_chunk().is_empty() {
                                chunk_handle.push_super_chunk(super_chunk, token);
                            }
                        });
                    }

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