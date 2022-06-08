use protocol::{PlayerCoord, Coord};
use crate::channel::*;
use protocol::event::*;
use protocol::Token;
use crate::chunks::ChunkHandle;
use std::collections::BTreeSet;
use protocol::chunk::CHUNK_SIZE;
use crate::CONFIG;

pub(super) struct ChunkLoader {
    chunk_handle: ChunkHandle,
    tcp_sender: Sender<Event>,
    last_load_pos: PlayerCoord,
    chunks: BTreeSet<Coord>
}
impl ChunkLoader {
    pub fn new(
        chunk_handle: ChunkHandle,
        tcp_sender: Sender<Event>,
    ) -> Self {
        let last_load_pos = [0.0, 0.0, 0.0];
        ChunkLoader{
            chunk_handle,
            tcp_sender,
            last_load_pos,
            chunks: BTreeSet::default(),
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
        let distance: f64 =
            protocol::calculate_distance(&pos, &self.last_load_pos);
        if distance as usize > CHUNK_SIZE {
            self.last_load_pos = pos;

            let origin = protocol::chunk::get_chunk_coords(&[
                pos[0] as i64,
                pos[1] as i64,
                pos[2] as i64,
            ]).0;

            let mut chunk_coords: Vec<Coord> = Vec::new();

            let offset = CONFIG.chunks.render_distance as i64;
            for x in origin[0] - offset..=origin[0] + offset {
                for y in origin[1] - offset..=origin[1] + offset {
                    for z in origin[2] - offset..=origin[2] + offset {
                        let coord = [x, y, z];
                        if !self.chunks.contains(&coord) {
                            chunk_coords.push([x, y, z]);
                            self.chunks.insert(coord);
                        }
                    }
                }
            }
            chunk_coords.sort_unstable_by_key(|key| {
                protocol::calculate_chunk_distance(key, &origin)
            });
            for chunk_coords in chunk_coords.chunks(64) {
                if let Some(chunks) = self.chunk_handle.request_chunks(chunk_coords.to_vec(), token) {
                    for chunk in chunks {
                        let _ = self.tcp_sender.send(Event::ServerToClient(ServerToClient::ChunkUpdate(chunk)));
                    }
                }
            }
        }
    }
}