use super::generation::generate;
use super::Instruction;
use super::StoredChunk;
use crate::CONFIG;
use protocol::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::thread;
use Instruction::*;

#[rustfmt::skip]
fn send_load_chunks(
    chunk: &Chunk,
    load_requests: &mut HashMap<
        Token,
        (
            BTreeSet<Coord>,
            Sender<Chunk>,
        ),
    >,
    important: bool,
) {
    // INFO: BLOCK BREAKING MUST USE AIR INSTEAD OF NONE
    // otherwise this will block chunk-updates
    if chunk.is_empty() {
        return;
    }

    // send main_chunk to all reqester if requested
    for (token, (
        load_coords,
        load_sender,
    )) in load_requests.clone().iter()
    {
        if load_coords.contains(&chunk.coord) {
            // if true client disconnected
            if load_sender.send(chunk.clone(), important).is_err() {
                log::info!("removed chunk sender");
                load_requests.remove(token);
                continue;
            }
            // to not send chunkload multiple times to same client
            load_requests.get_mut(token).unwrap().0.remove(&chunk.coord);
        }
    }
}

#[rustfmt::skip]
fn send_update_chunks(
    pre_chunk: &StoredChunk,
    chunk: &Chunk,
    update_requests: &mut HashMap<
        Token,
        (
            BTreeSet<Coord>,
            Sender<ChunkDelta>,
        ),
    >,
    important: bool,
) {
    for (token, (
        update_coords,
        update_sender,
    )) in update_requests.clone().iter() {
        if update_coords.contains(&chunk.coord) {
            let delta = pre_chunk.chunk.get_delta(&chunk);
            if !delta.1.is_empty() {
                if update_sender.send(delta, important).is_err() {
                    log::info!("removed chunk sender");
                    update_requests.remove(token);
                    continue;
                }
            }
        }
    }
}

// generates chunks nonblockingly
// INFO: leftover do get leaked and not unspawned if they get generated outside of view of client
pub(super) fn init(rx: Receiver<Instruction>, chunk_handle: super::ChunkHandle) {
    thread::spawn(move || {
        let threadpool = threadpool::ThreadPool::new(4);

        let mut chunk_queque: BTreeSet<Coord> = BTreeSet::default();
        let mut chunks: BTreeMap<Coord, StoredChunk> = BTreeMap::default();
        let mut leftover: BTreeMap<Coord, StoredChunk> = BTreeMap::default();

        let mut load_requests: HashMap<
            Token,
            (
                BTreeSet<Coord>,
                Sender<Chunk>,
            ),
        > = HashMap::default();

        let mut update_requests: HashMap<
            Token,
            (
                BTreeSet<Coord>,
                Sender<ChunkDelta>,
            ),
        > = HashMap::default();

        while let Ok(instruction) = rx.recv() {
            match instruction {
                // INFO: main bottleneck in whole chunkhandle
                PushSuperChunk {
                    mut super_chunk,
                    token,
                    important,
                } => {
                    /* --- PHASE 1 --- */
                    // extract main_chunk and merge it with leftover
                    let mut main_chunk = super_chunk.remove_main_chunk();
                    chunk_queque.remove(&main_chunk.coord);

                    if chunks.contains_key(&main_chunk.coord) {log::warn!("invalid chunk generation")}
                    
                    if let Some(left) = leftover.remove(&main_chunk.coord) {
                        main_chunk.merge(&left.chunk);
                    }

                    /* --- PHASE 2 --- */
                    // push main_chunk to chunks and mark it as needed
                    if !main_chunk.is_empty() {
                        send_load_chunks(&main_chunk, &mut load_requests, important);
                        let mc_coord = main_chunk.coord;
                        let mut stored_chunk = StoredChunk::new(main_chunk);
                        stored_chunk.mark_needed_by(token);
                        chunks.insert(mc_coord, stored_chunk);
                    }

                    /* --- PHASE 3 --- */
                    // handle leftovers
                    for (coord, left) in super_chunk.chunks.into_iter() {
                        if let Some(already_left) = leftover.get_mut(&coord) {
                            already_left.chunk.merge(&left);
                            already_left.mark_needed_by(token);
                        } else {
                            let mut left = StoredChunk::new(left.clone());
                            left.mark_needed_by(token);
                            leftover.insert(coord, left);
                        }

                        // when generated chunk is found at leftover coord merge it with leftover
                        if let Some(stored_chunk) = chunks.get_mut(&coord) {
                            stored_chunk.mark_needed_by(token);
                            // guaranteed not to panic because of above code
                            let pre_chunk = stored_chunk.clone();
                            let left = leftover.remove(&coord).unwrap();
                            stored_chunk.chunk.merge(&left.chunk);
                            // send_chunk_to_requester(Some(&pre_chunk), &stored_chunk.chunk, &mut requests, false, important);
                            send_load_chunks(&stored_chunk.chunk, &mut load_requests, important);
                            send_update_chunks(&pre_chunk, &stored_chunk.chunk, &mut update_requests, important);
                        } else {
                            // send leftover to client
                            // send_chunk_to_requester(Some(&StoredChunk::new(Chunk::new(coord))), &left, &mut requests, true, important);
                            send_update_chunks(&StoredChunk::new(Chunk::new(coord)), &left, &mut update_requests, important);
                        }
                    }
                }
                SetUpdateChunks {
                    update_coords,
                    token,
                    update_sender
                } => {
                    let mut u_coords: BTreeSet<Coord> = BTreeSet::default();
                    for update_coord in update_coords.iter() {
                        u_coords.insert(*update_coord);
                    }
                    update_requests.insert(token, (u_coords, update_sender));
                }
                
                PushLoadChunks {
                    load_coords,
                    load_sender,
                    token,
                } => {
                    // push rather than overwrite, VERY IMPORTANT
                    if let Some(load_requests) = load_requests.get_mut(&token) {
                        for load_coord in load_coords.iter() {
                            load_requests.0.insert(*load_coord);
                        }
                        load_requests.1 = load_sender.clone();
                    } else {
                        // init coords
                        let mut l_coords: BTreeSet<Coord> = BTreeSet::default();
                        for load_coord in load_coords.iter() {
                            l_coords.insert(*load_coord);
                        }
                        load_requests.insert(token, (l_coords, load_sender.clone()));
                    }

                    // request generation of not already loaded chunks
                    for coord in load_coords.into_iter() {
                        if !chunks.contains_key(&coord) && !chunk_queque.contains(&coord) {
                            chunk_queque.insert(coord);
                            // || leftover.contains_key(&coord) {
                            let chunk_handle = chunk_handle.clone();
                            threadpool.execute(move || {
                                let super_chunk = generate(Chunk::new(coord), CONFIG.chunks.seed);
                                chunk_handle.push_super_chunk(super_chunk, token, false);
                            });
                        } else {
                            if let Some(chunk) = chunks.get_mut(&coord) {
                                if let Some(left) = leftover.get(&coord) {
                                    chunk.chunk.merge(&left.chunk);
                                }
                                let _ = load_sender.send(chunk.chunk.clone(), false).unwrap();
                            }
                        }
                    }
                }
                RequestUnloadChunk { coords, token } => {
                    for coord in coords.iter() {
                        // chunk
                        if let Some(chunk) = chunks.get_mut(coord) {
                            chunk.mark_unneeded_by(&token);
                            if !chunk.is_needed() {
                                chunks.remove(coord);
                            }
                        }
                        // leftover
                        if let Some(left) = leftover.get_mut(coord) {
                            left.mark_unneeded_by(&token);
                            if !left.is_needed() {
                                leftover.remove(coord);
                            }
                        }

                    }
                }
                Instruction::PlaceStructure { coord, structure, token } => {
                    let mut super_chunk = SuperChunk::new(Chunk::new(coord));
                    super_chunk.place_structure(&structure, coord);
                    chunk_handle.push_super_chunk(super_chunk, token, true);
                }
            }
        }
        log::warn!("unexpected shutdown of ChunkHandle");
    });
}
