use protocol::Coord;
use super::{Handle, Chunk};
use std::collections::HashMap;
use super::super::generation;

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

pub async fn init_load_requester(handler: Handle) {
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1000));

        loop {
            interval.tick().await;
            handler.flush_load_queue().await
        }
    });

    forever.await.unwrap();
}