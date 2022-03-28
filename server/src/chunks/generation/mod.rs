use noise::{OpenSimplex, NoiseFn, Seedable};

use super::CHUNK_SIZE;
use protocol::chunk::{Chunk, SuperChunk};
use protocol::chunk::Block;

pub fn generate(chunk: Chunk, seed: u32) -> SuperChunk {
    let key = chunk.coord;
    //let tree_struct = structures::generate_tree();
    let noise = OpenSimplex::new();
    noise.set_seed(seed);

    let mut chunks: SuperChunk = SuperChunk::new( (key, chunk) );
    /* landscape */
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            /*
            let global_x = (key[0] * CHUNK_SIZE as i64) + x as i64;
            let global_z = (key[2] * CHUNK_SIZE as i64) + z as i64;

            let mut height = (noise.get([global_x as f64 * 0.013, global_z as f64 * 0.013]) + 1.0) * 70.0;
            height += noise.get([global_x as f64 * 0.04, global_z as f64 * 0.04]) * 2.0;
            height += noise.get([global_x as f64 * 0.043, global_z as f64 * 0.043]) * 14.5;
            
            let height = height as i64;

            for height in 0..height {
                if height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64 && height >= key[1] * CHUNK_SIZE as i64 {
                    // grass
                    //for height in 0..height as i64 - 1 {
                    //    chunks.place([global_x, height, global_z], Block::Dirt);
                    //}
                    chunks.place([global_x, height, global_z], Block::Grass);
                }
            }
            */
            let global_x = (key[0] * CHUNK_SIZE as i64) + x as i64;
            let global_z = (key[2] * CHUNK_SIZE as i64) + z as i64;
            
            chunks.place([global_x, 4, global_z], Block::Grass);
        }
    }

    /* 3d 
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let global_x = (key[0] * CHUNK_SIZE as i64) + x as i64;
                let global_y = (key[1] * CHUNK_SIZE as i64) + y as i64;
                let global_z = (key[2] * CHUNK_SIZE as i64) + z as i64;

                let cave = noise.get([
                    global_x as f64 * 0.022,
                    global_y as f64 * 0.042,
                    global_z as f64 * 0.022,
                ]);
                if cave > -0.0 {
                    chunks.place([global_x, global_y, global_z], Block::Dirt);
                }
            }
        }
    }
    */

    chunks
}