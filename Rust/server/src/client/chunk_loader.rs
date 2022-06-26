use crate::chunks::ChunkHandle;
use crate::CONFIG;
use protocol::prelude::*;
use std::collections::BTreeSet;

pub(super) struct ChunkLoader {
    chunk_handle: ChunkHandle,
    tcp_sender: Sender<STC>,
    last_load_pos: Coord,
    chunk_load_sender: Sender<Chunk>,
    chunk_load_receiver: Receiver<Chunk>,
    chunk_update_sender: Sender<ChunkDelta>,
    chunk_update_receiver: Receiver<ChunkDelta>,
    chunks: BTreeSet<Coord>,
    leftover: BTreeSet<Coord>,
    chunk_pos: Coord,
    pub moved: bool,
    chunk_load_queque: Vec<Coord>,
}
impl ChunkLoader {
    pub fn new(chunk_handle: ChunkHandle, tcp_sender: Sender<STC>) -> Self {
        let last_load_pos = [0, 0, 0];
        let (chunk_load_sender, chunk_load_receiver) = channel(None);
        let (chunk_update_sender, chunk_update_receiver) = channel(None);
        ChunkLoader {
            chunk_handle,
            tcp_sender,
            last_load_pos,
            chunk_load_sender,
            chunk_load_receiver,
            chunk_update_sender,
            chunk_update_receiver,
            chunks: BTreeSet::default(),
            leftover: BTreeSet::default(),
            chunk_pos: [0, 0, 0],
            moved: true,
            chunk_load_queque: Vec::new(),
        }
    }

    pub fn set_player_pos(&mut self, pos: PlayerCoord) {
        let mut coord = get_chunk_coords(&player_coord_to_coord(pos)).0;
        coord[1] += 1;
        self.last_load_pos = coord;
    }

    pub fn update_position(&mut self, pos: PlayerCoord) {
        let chunk_pos =
            protocol::chunk::get_chunk_coords(&[pos[0] as i64, pos[1] as i64, pos[2] as i64]).0;
        self.chunk_pos = chunk_pos;

        let distance = calculate_chunk_distance(&chunk_pos, &self.last_load_pos);
        if distance >= 1 {
            self.moved = true;
            self.last_load_pos = chunk_pos;
        }
    }

    pub fn calculate_chunks(&mut self, token: Token) {
        if self.moved {
            /* --- LOAD --- */
            let mut chunk_update_coords: Vec<Coord> = Vec::new();
            let offset = CONFIG.chunks.render_distance as i64;
            for x in self.chunk_pos[0] - offset..=self.chunk_pos[0] + offset {
                for y in self.chunk_pos[1] / 2 - offset..=self.chunk_pos[1] / 2 + offset {
                    for z in self.chunk_pos[2] - offset..=self.chunk_pos[2] + offset {
                        let coord = [x, y, z];
                        if protocol::calculate_chunk_distance(&self.chunk_pos, &coord) < offset {
                            if !self.chunks.contains(&coord) && !self.chunk_load_queque.contains(&coord) {
                                self.chunk_load_queque.push(coord);
                                chunk_update_coords.push(coord);
                            } else {
                                chunk_update_coords.push(coord);
                            }
                        }
                    }
                }
            }
            if !self.chunk_load_queque.is_empty() || !chunk_update_coords.is_empty() {
                chunk_update_coords
                    .sort_unstable_by_key(|key| protocol::calculate_chunk_distance(key, &self.chunk_pos));

                self.chunk_load_queque
                    .sort_unstable_by_key(|key| protocol::calculate_chunk_distance(key, &self.chunk_pos));

                self.chunk_handle.set_update_chunks(chunk_update_coords, self.chunk_update_sender.clone(), token);
            }
        }
    }

    pub fn request_chunks(&mut self, token: Token) {
        // sort and send the chunks, which are marked to load to chunk handle
        if !self.chunk_load_queque.is_empty() {
            let coord_amount = CONFIG.chunks.chunks_per_cycle as usize;
            let coords = if self.chunk_load_queque.len() > coord_amount {
                self.chunk_load_queque.drain(0..coord_amount)
            } else {
                self.chunk_load_queque.drain(0..)
            }.as_slice().to_vec();
            self.chunk_handle.push_load_chunks(coords, self.chunk_load_sender.clone(), token);
        }
    }

    pub fn unload(&mut self, token: Token) {
        // load and unload chunks
        if self.moved {
            /* --- UNLOAD --- */
            let mut to_unload: Vec<Coord> = Vec::new();
            for chunk in self.chunks.clone().into_iter() {
                if protocol::calculate_chunk_distance(&chunk, &self.chunk_pos) > CONFIG.chunks.render_distance as i64 + 1 {
                    self.chunks.remove(&chunk);
                    to_unload.push(chunk);
                }
            }
            for chunk in self.leftover.clone().into_iter() {
                if protocol::calculate_chunk_distance(&chunk, &self.chunk_pos) > CONFIG.chunks.render_distance as i64 + 1 {
                    self.chunks.remove(&chunk);
                    to_unload.push(chunk);
                }
            }
            if !to_unload.is_empty() {
                self.chunk_handle.unload_chunks(to_unload.clone(), token);
                self
                    .tcp_sender
                    .send(STC::ChunkUnloads(to_unload), false)
                    .unwrap();
            }
        }
    }

    pub fn receive_chunks(&mut self) {
        // Receiving and sending to client
        let mut chunk_loads: Vec<Chunk> = Vec::new();
        let mut chunk_updates: Vec<ChunkDelta> = Vec::new();
        // chunk load
        let mut sent = 0;
        while let Ok(chunk) = self.chunk_load_receiver.try_recv() {
            if protocol::calculate_chunk_distance(&self.chunk_pos, &chunk.coord) < CONFIG.chunks.render_distance as i64 {
                self.chunks.insert(chunk.coord);
                chunk_loads.push(chunk);
                sent += 1;
                if sent > CONFIG.chunks.chunks_per_cycle / 2 {
                    break;
                }
            }
        }
        // chunk update
        let mut sent = 0;
        while let Ok((chunk, important)) = self.chunk_update_receiver.try_recv_with_meta() {
            if important && protocol::calculate_chunk_distance(&self.chunk_pos, &chunk.0) < CONFIG.chunks.render_distance as i64 {
                self.leftover.insert(chunk.0);
                self
                .tcp_sender
                    .send(STC::ChunkUpdates(vec![chunk]), true)
                    .unwrap();
                sent += 1;
                if sent > CONFIG.chunks.chunks_per_cycle / 2 {
                    break;
                } 
            } else if protocol::calculate_chunk_distance(&self.chunk_pos, &chunk.0) < CONFIG.chunks.render_distance as i64 {
                self.leftover.insert(chunk.0);
                chunk_updates.push(chunk);
                sent += 1;
                if sent > CONFIG.chunks.chunks_per_cycle / 2 {
                    break;
                }
            }
        }

        if !chunk_loads.is_empty() {
            self
                .tcp_sender
                .send(STC::ChunkLoads(chunk_loads.clone()), false)
                .unwrap();
        }
        if !chunk_updates.is_empty() {
            self
                .tcp_sender
                .send(STC::ChunkUpdates(chunk_updates.clone()), false)
                .unwrap();
        }
    }

    pub fn unload_all_chunks(&mut self, token: Token) {
        let mut to_unload: Vec<Coord> = Vec::new();
        for chunk in self.chunks.clone().into_iter() {
            self.chunks.remove(&chunk);
            to_unload.push(chunk);
        }
        for chunk in self.leftover.clone().into_iter() {
            self.chunks.remove(&chunk);
            to_unload.push(chunk);
        }
        if !to_unload.is_empty() {
            self.chunk_handle.unload_chunks(to_unload.clone(), token);
        }
    }
}

// for clear_duplicates()
trait Dedup<T: PartialEq + Clone> {
    fn clear_duplicates(&mut self);
}

impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
    fn clear_duplicates(&mut self) {
        let mut already_seen = Vec::new();
        self.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        })
    }
}
