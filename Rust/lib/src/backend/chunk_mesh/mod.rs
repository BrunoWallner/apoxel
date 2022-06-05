mod gen_chunk;
mod gen_mesh;


use protocol::chunk::Chunk;
use protocol::chunk::Block;
use protocol::chunk::CHUNK_SIZE;
use protocol::Coord;
use std::collections::HashMap;
use std::thread;

use crossbeam::channel;
use crate::terminator::Terminator;

use gdnative::prelude::{Ref, Spatial};

// side anordnung: left, right, front, back, top, bottom

pub fn init_generation(
    chunk_receiver: channel::Receiver<Chunk>,
    chunk_mesh_sender: channel::Sender<Option<Ref<Spatial>>>,
    terminator: Terminator,
) {
    thread::spawn(move || {
        let threadpool = threadpool::ThreadPool::new(2);
        let mut chunk_map: HashMap<Coord, Chunk> = HashMap::default();
        loop {
            if terminator.should_terminate() {
                break;
            }
            if let Ok(chunk) = chunk_receiver.recv() {
                chunk_map.insert(chunk.coord, chunk);
                // get sides of surrounding chunks
                // under 900 µs
                let mut sides: [Option<[[Block; CHUNK_SIZE]; CHUNK_SIZE]>; 6] = [None; 6];
                let coord = chunk.coord;
                // left side
                sides[0] = chunk_map.get(&[coord[0] - 1, coord[1], coord[2]]).map(|c| c.get_right_side());
                // right side
                sides[1] = chunk_map.get(&[coord[0] + 1, coord[1], coord[2]]).map(|c| c.get_left_side());
                // front side
                sides[2] = chunk_map.get(&[coord[0], coord[1], coord[2] + 1]).map(|c| c.get_back_side());
                // back side
                sides[3] = chunk_map.get(&[coord[0], coord[1], coord[2] - 1]).map(|c| c.get_front_side());
                // top side
                sides[4] = chunk_map.get(&[coord[0], coord[1] + 1, coord[2]]).map(|c| c.get_bottom_side());
                // bottom side
                sides[5] = chunk_map.get(&[coord[0], coord[1] - 1, coord[2]]).map(|c| c.get_top_side());

                let sender = chunk_mesh_sender.clone();
                threadpool.execute(move || {
                    let mesh = gen_mesh::gen(chunk, sides);
                    let chunk = gen_chunk::gen(mesh); // 120ms
                    let _ = sender.send(chunk);
                });
            }
        }
    });
}