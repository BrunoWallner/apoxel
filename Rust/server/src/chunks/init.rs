use crate::channel::*;
use super::Instruction;
use Instruction::*;
use std::thread;
use std::collections::{BTreeMap, HashMap, BTreeSet};
use protocol::{Coord, chunk::Chunk, Token};
use super::generation::generate;
use crate::CONFIG;
use super::StoredChunk;

fn send_chunk_to_requester(
    chunk: &Chunk,
    requests: &mut HashMap<Token, (BTreeSet<Coord>, Sender<Chunk>)>,
) {
    // send main_chunk to all reqester if requested
    for (token, (coords, sender)) in requests.clone().iter() {
        if coords.contains(&chunk.coord) {
            // if true client disconnected
            if sender.send(chunk.clone()).is_err() {requests.remove(token);}
        }
    }
}

// generates chunks nonblockingly
pub(super) fn init(
    rx: Receiver<Instruction>,
    chunk_handle: super::ChunkHandle,
) {
    thread::spawn(move || {
        let threadpool = threadpool::ThreadPool::new(2);

        let mut chunks: BTreeMap<Coord, StoredChunk> = BTreeMap::default();
        let mut leftover: BTreeMap<Coord, Chunk> = BTreeMap::default();

        let mut requests: HashMap<Token, (BTreeSet<Coord>, Sender<Chunk>)> = HashMap::default();

        while let Some(instruction) = rx.recv() {
            match instruction {
                // INFO: main bottleneck in whole chunkhandle
                PushSuperChunk{mut super_chunk, token} => {
                    /* --- PHASE 1 --- */
                    // extract main_chunk and merge it with leftover
                    let mut main_chunk = super_chunk.remove_main_chunk();
                    if let Some(left) = leftover.remove(&main_chunk.coord) {
                        main_chunk.merge(&left);
                    }
                    if !main_chunk.is_empty() {
                        send_chunk_to_requester(&main_chunk, &mut requests);
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
                            send_chunk_to_requester(&stored_chunk.chunk, &mut requests);
                        }
                    }
                }
                // INFO! only properly works, when already in chunk register
                RequestChunks { coords, token, sender } => {
                    // filtering already generated coords and send them
                    let mut new_coords: BTreeSet<Coord> = BTreeSet::default();
                    for coord in coords.iter() {
                        new_coords.insert(*coord);
                    }
                    requests.insert(token, (new_coords.clone(), sender));

                    for coord in new_coords.into_iter() {
                        if !chunks.contains_key(&coord) { // || leftover.contains_key(&coord) {
                            let chunk_handle = chunk_handle.clone();
                            threadpool.execute(move || {
                                let super_chunk = generate(Chunk::new(coord), CONFIG.chunks.seed);
                                chunk_handle.push_super_chunk(super_chunk, token);
                            });
                        }
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