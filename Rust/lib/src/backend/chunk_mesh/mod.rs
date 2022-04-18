mod gen_chunk;
mod gen_mesh;
mod sides;


use protocol::chunk::Chunk;
use protocol::chunk::Block;
use protocol::chunk::CHUNK_SIZE;
use protocol::Coord;
use std::collections::HashMap;
use std::thread;

use crossbeam::channel;

use gdnative::prelude::{Ref, Spatial};

// side anordnung: left, right, front, back, top, bottom

pub fn init_generation(
    chunk_receiver: channel::Receiver<Chunk>,
    chunk_mesh_sender: channel::Sender<Option<Ref<Spatial>>>,
) {
    let pool = threadpool::ThreadPool::new(8);
    thread::spawn(move || {
        let mut chunk_map: HashMap<Coord, Chunk> = HashMap::default();

        loop {
            let chunk = chunk_receiver.recv().unwrap();
            chunk_map.insert(chunk.coord, chunk);
            // get sides of surrounding chunks
            let mut sides: [Option<[[Block; CHUNK_SIZE]; CHUNK_SIZE]>; 6] = [None; 6];
            let coord = chunk.coord;
            // left side
            sides[0] = match chunk_map.get(&[coord[0] - 1, coord[1], coord[2]]) {
                Some(c) => Some(c.get_right_side()),
                None => None,
            };
            // right side
            sides[1] = match chunk_map.get(&[coord[0] + 1, coord[1], coord[2]]) {
                Some(c) => Some(c.get_left_side()),
                None => None,
            };
            // front side
            sides[2] = match chunk_map.get(&[coord[0], coord[1], coord[2] + 1]) {
                Some(c) => Some(c.get_back_side()),
                None => None,
            };
            // back side
            sides[3] = match chunk_map.get(&[coord[0], coord[1], coord[2] - 1]) {
                Some(c) => Some(c.get_front_side()),
                None => None,
            };
            // top side
            sides[4] = match chunk_map.get(&[coord[0], coord[1] + 1, coord[2]]) {
                Some(c) => Some(c.get_bottom_side()),
                None => None,
            };
            // bottom side
            sides[5] = match chunk_map.get(&[coord[0], coord[1] - 1, coord[2]]) {
                Some(c) => Some(c.get_top_side()),
                None => None,
            };

            let sender = chunk_mesh_sender.clone();
            pool.execute(move || {
                let mesh = gen_mesh::gen(chunk, sides);
                let chunk = gen_chunk::gen(mesh);
                sender.send(chunk.clone()).unwrap();
            });
        }
    });
}