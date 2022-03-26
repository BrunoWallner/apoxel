use tokio::{sync::mpsc, task, time};
use std::time::Duration;
use crate::broadcaster::BroadCaster;
use crate::events;
use protocol::{Token, Coord, PlayerCoord};
use crate::player::handle::Handle as PlayerHandle;
use crate::chunks::handle::Handle as ChunkHandle;
use std::sync::{Arc, Mutex};

use crate::config::CONFIG;
use protocol::event::Event;

struct PlayerPos {
    pos: PlayerCoord,
}
impl PlayerPos {
    fn set(&mut self, coord: PlayerCoord) {
        self.pos = coord;
    }
}

pub async fn init(
    mut receiver: mpsc::Receiver<Coord>,
    write_broadcaster: BroadCaster<events::Tcp>,
    player_handle: PlayerHandle,
    chunk_handle: ChunkHandle,
    token: Token,
) {
    let player_pos = Arc::new(Mutex::new(PlayerPos{pos: [0.0, 0.0, 0.0]}));

    let player_pos_clone = player_pos.clone();
    tokio::spawn(async move {
        loop {
            let pos = player_pos_clone.lock().unwrap().pos;
            let pos = protocol::chunk::get_chunk_coords(&[pos[0] as i64, pos[1] as i64, pos[2] as i64]).0;
            let coord = receiver.recv().await.unwrap();

            // check if coord is in range of player
            let distance = (( 
                (pos[0] - coord[0]).pow(2) +
                (pos[1] - coord[1]).pow(2) +
                (pos[2] - coord[2]).pow(2)
            ) as f64).sqrt() as i64;

            if distance <= CONFIG.chunks.render_distance as i64 {
                let chunk = chunk_handle.request(vec![coord]).await[0];
                write_broadcaster.send(events::Tcp::Protocol(Event::ChunkUpdate(chunk))).await;
            }
        }
    });

    let player_pos_fetcher = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(500));

        loop {
            interval.tick().await;
            if let Some(player) = player_handle.get_player(token).await {
                let mut pp = player_pos.lock().unwrap();
                pp.set(player.pos);
            }
        }
    });

    player_pos_fetcher.await.unwrap();
}