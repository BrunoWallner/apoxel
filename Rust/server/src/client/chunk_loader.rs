use crate::channel::*;
use crate::chunks::ChunkHandle;
use crate::CONFIG;
use protocol::chunk::CHUNK_SIZE;
use protocol::chunk::Chunk;
use protocol::event::*;
use protocol::Token;
use protocol::{Coord, PlayerCoord};
use std::collections::BTreeSet;

pub(super) struct ChunkLoader {
    chunk_handle: ChunkHandle,
    tcp_sender: Sender<Event>,
    last_load_pos: PlayerCoord,
    chunk_receiver: Receiver<Chunk>,
    chunk_sender: Sender<Chunk>,
}
impl ChunkLoader {
    pub fn new(
        chunk_handle: ChunkHandle,
        tcp_sender: Sender<Event>,
    ) -> Self {
        let last_load_pos = [0.0, 0.0, 0.0];
        let (chunk_sender, chunk_receiver) = channel();
        ChunkLoader {
            chunk_handle,
            tcp_sender,
            last_load_pos,
            chunk_sender,
            chunk_receiver,
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
        let mut chunk_coords: Vec<Coord> = Vec::new();
        let origin =
            protocol::chunk::get_chunk_coords(&[pos[0] as i64, pos[1] as i64, pos[2] as i64]).0;

        let distance: f64 = protocol::calculate_distance(&pos, &self.last_load_pos);
        if distance as usize > CHUNK_SIZE {
            self.last_load_pos = pos;
            let offset = CONFIG.chunks.render_distance as i64;
            for x in origin[0] - offset..=origin[0] + offset {
                for y in origin[1] - offset..=origin[1] + offset {
                    for z in origin[2] - offset..=origin[2] + offset {
                        let coord = [x, y, z];
                        if protocol::calculate_chunk_distance(&origin, &coord) < offset {
                            chunk_coords.push(coord);
                        } 
                    }
                }
            }
        }
        if !chunk_coords.is_empty() {
            chunk_coords
                .sort_unstable_by_key(|key| protocol::calculate_chunk_distance(key, &origin));

            self.chunk_handle.request_chunks(chunk_coords, token, self.chunk_sender.clone());
        }

        // INFO: ONLY ONE CHUNKUPDATE PER CYCLE
        for _ in 0..25 {
            if let Some(chunk) = self.chunk_receiver.try_recv() {
                let _ = self
                    .tcp_sender
                    .send(Event::ServerToClient(ServerToClient::ChunkUpdate(chunk)));
            } else {
                break
            }
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
