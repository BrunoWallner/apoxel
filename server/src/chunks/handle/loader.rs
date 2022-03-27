use protocol::Coord;
use super::Handle;
use protocol::chunk::Chunk;
use std::collections::HashMap;
use super::super::generation;
use crate::player;
use super::coord_converter;

use crate::config::CONFIG;

pub async fn load(
    coords: Vec<Coord>,
    handler: Handle,
) {
    let mut chunks: HashMap<Coord, Chunk> = HashMap::default();

    let loaded = handler.check_if_loaded(coords.clone()).await;

    for (i, coord) in coords.iter().enumerate() {
        if !loaded[i] {
            let chunk = if !chunks.contains_key(coord) {
                Chunk::new(*coord)
            } else {
                *chunks.get(coord).unwrap()
            };

            let super_chunk = generation::generate(chunk, 93845709);
            for (key, data) in super_chunk.chunks.iter() {
                //voxels.map.insert(*key, *data);
                if let Some(voxels) = chunks.get_mut(key) {
                    voxels.merge(data);
                } else {
                    chunks.insert(*key, *data);
                }
            }
        }
    }
    handler.push_chunks(chunks).await;
}

use std::time::Duration;
use tokio::{task, time};

pub async fn player_chunk_loader(chunk_handle: Handle, player_handle: player::handle::Handle) {
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(100));

        loop {
            interval.tick().await;
            
            let p_coords = player_handle.get_coords().await;
            let p_coords = coord_converter(p_coords);

            let rd = CONFIG.chunks.render_distance as i64;
            let mut load_coords: Vec<[i64; 3]> = Vec::new();
            for coord in p_coords.iter() {
                for x in -rd..rd {
                    for y in -rd..rd {
                        for z in -rd..rd {
                            if x.pow(2) + y.pow(2) + z.pow(2) <= rd.pow(2) {
                                load_coords.push([
                                    coord[0] + x,
                                    coord[1] + y,
                                    coord[2] + z
                                ]);
                            }
                        }
                    }
                }
            }
            // clears already loaded chunks
            let loaded = chunk_handle.check_if_loaded(load_coords.clone()).await;
            let mut cleared: usize = 0;
            for (i, loaded) in loaded.iter().enumerate() {
                if *loaded {
                    load_coords.remove(i - cleared);
                    cleared += 1;
                }
            }

            chunk_handle.load(load_coords).await;
        }
    });

    forever.await.unwrap();
}

pub async fn init_flusher(handler: Handle) {
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(10));

        loop {
            interval.tick().await;
            handler.flush_load_queue().await
        }
    });

    forever.await.unwrap();
}