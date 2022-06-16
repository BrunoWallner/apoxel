use crate::channel::*;
use crate::chunks::ChunkHandle;
use crate::CONFIG;
use protocol::chunk::Chunk;
use protocol::chunk::CHUNK_SIZE;
use protocol::event::*;
use protocol::Token;
use protocol::prelude::ChunkDelta;
use protocol::{Coord, PlayerCoord};
use std::collections::BTreeSet;
use crate::queque::Queque;

pub(super) struct ChunkLoader {
    chunk_handle: ChunkHandle,
    tcp_sender: Sender<Event>,
    last_load_pos: PlayerCoord,
    chunk_load_queque: Queque<Chunk>,
    chunk_update_queque: Queque<ChunkDelta>,
    chunks: BTreeSet<Coord>,
    leftover: BTreeSet<Coord>,
}
impl ChunkLoader {
    pub fn new(chunk_handle: ChunkHandle, tcp_sender: Sender<Event>) -> Self {
        let last_load_pos = [0.0, 0.0, 0.0];
        let chunk_load_queque = Queque::new();
        let chunk_update_queque = Queque::new();
        ChunkLoader {
            chunk_handle,
            tcp_sender,
            last_load_pos,
            chunk_load_queque,
            chunk_update_queque,
            chunks: BTreeSet::default(),
            leftover: BTreeSet::default(),
        }
    }

    pub fn set_player_pos(&mut self, pos: PlayerCoord) {
        self.last_load_pos = [
            pos[0],
            pos[1] + CHUNK_SIZE as f64 + 1.0, // will trigger chunk loading
            pos[2],
        ];
    }

    pub fn update_position(&mut self, pos: PlayerCoord, token: Token) {
        let origin =
            protocol::chunk::get_chunk_coords(&[pos[0] as i64, pos[1] as i64, pos[2] as i64]).0;

        // load and unload chunks
        let mut chunk_load_coords: Vec<Coord> = Vec::new();
        let mut chunk_update_coords: Vec<Coord> = Vec::new();
        let distance: f64 = protocol::calculate_distance(&pos, &self.last_load_pos);
        if distance as usize > CHUNK_SIZE {
            self.last_load_pos = pos;

            /* --- UNLOAD --- */
            let mut to_unload: Vec<Coord> = Vec::new();
            for chunk in self.chunks.clone().into_iter() {
                if protocol::calculate_chunk_distance(&chunk, &origin) > CONFIG.chunks.render_distance as i64 + 1 {
                    self.chunks.remove(&chunk);
                    to_unload.push(chunk);
                }
            }
            for chunk in self.leftover.clone().into_iter() {
                if protocol::calculate_chunk_distance(&chunk, &origin) > CONFIG.chunks.render_distance as i64 + 1 {
                    self.chunks.remove(&chunk);
                    to_unload.push(chunk);
                }
            }
            if !to_unload.is_empty() {
                self.chunk_handle.unload_chunks(to_unload.clone(), token);
                self
                    .tcp_sender
                    .send(Event::ServerToClient(ServerToClient::ChunkUnloads(to_unload)))
                    .unwrap();
            }

            /* --- LOAD --- */
            let offset = CONFIG.chunks.render_distance as i64;
            for x in origin[0] - offset..=origin[0] + offset {
                for y in origin[1] - offset..=origin[1] + offset {
                    for z in origin[2] - offset..=origin[2] + offset {
                        let coord = [x, y, z];
                        if protocol::calculate_chunk_distance(&origin, &coord) < offset {
                            if !self.chunks.contains(&coord) {
                                chunk_load_coords.push(coord);
                                chunk_update_coords.push(coord);
                            } else {
                                chunk_update_coords.push(coord);
                            }
                        }
                    }
                }
            }

            // sort and send the chunks, which are marked to load to chunk handle
            if !chunk_load_coords.is_empty() || !chunk_update_coords.is_empty() {
                chunk_load_coords
                    .sort_unstable_by_key(|key| protocol::calculate_chunk_distance(key, &origin));
                chunk_update_coords
                    .sort_unstable_by_key(|key| protocol::calculate_chunk_distance(key, &origin));

                self.chunk_handle.request_chunks(chunk_load_coords, chunk_update_coords, self.chunk_load_queque.clone(), self.chunk_update_queque.clone(), token);
            }
        }

        // Receiving and sending to client
        let mut chunk_loads: Vec<Chunk> = Vec::new();
        let mut chunk_updates: Vec<ChunkDelta> = Vec::new();
        // chunk load
        let mut sent = 0;
        while let Some(chunk) = self.chunk_load_queque.try_recv() {
            if protocol::calculate_chunk_distance(&origin, &chunk.coord) < CONFIG.chunks.render_distance as i64 {
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
        while let Some(chunk) = self.chunk_update_queque.try_recv() {
            if protocol::calculate_chunk_distance(&origin, &chunk.0) < CONFIG.chunks.render_distance as i64 {
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
                .send(Event::ServerToClient(ServerToClient::ChunkLoads(chunk_loads.clone())))
                .unwrap();
        }
        if !chunk_updates.is_empty() {
            self
                .tcp_sender
                .send(Event::ServerToClient(ServerToClient::ChunkUpdates(chunk_updates.clone())))
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
