use crate::player;
use super::Handle;
use protocol::{Coord, PlayerCoord};
use super::coord_converter;

use std::time::Duration;
use tokio::{task, time};

pub async fn init_unloader(chunk_handle: Handle, player_handle: player::handle::Handle) {
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1000));

        loop {
            interval.tick().await;
            let p_coords = player_handle.get_coords().await;
            let p_coords = coord_converter(p_coords);
            let c_coords = chunk_handle.get_keys().await;
            unload(
                p_coords,
                c_coords,
                chunk_handle.clone(),
            ).await;
        }
    });

    forever.await.unwrap();
}

async fn unload(
    player_coords: Vec<Coord>,
    chunk_coords: Vec<Coord>,
    chunk_handle: Handle,
) {
    let mut to_unload: Vec<Coord> = Vec::new();

    for chunk in chunk_coords.iter() {
        // check if it is in view of anyone
        for player in player_coords.iter() {
            let mut in_view: bool = false;
            if player[0].pow(2) + player[2].pow(2) + player[1].pow(2) <= 10_i64.pow(2) {
                in_view = true;
            }
            if !in_view {
                to_unload.push(*chunk);
            }
        }
    }

    chunk_handle.unload(to_unload).await;
}