use noise::{OpenSimplex, NoiseFn, Seedable};
use protocol::chunk::Structure;

use super::CHUNK_SIZE;
use protocol::chunk::{Chunk, SuperChunk};
use protocol::chunk::Block;

pub fn generate(chunk: Chunk, seed: u32) -> SuperChunk {
    let key = chunk.coord;
    let flower_struct = generate_flower();
    let noise = OpenSimplex::new();
    noise.set_seed(seed);

    let mut chunks: SuperChunk = SuperChunk::new( chunk );
    /* Landscape */
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let global_x = (key[0] * CHUNK_SIZE as i64) + x as i64;
            let global_z = (key[2] * CHUNK_SIZE as i64) + z as i64;

            let mut height = (noise.get([global_x as f64 * 0.0033, global_z as f64 * 0.0033]) + 1.0) * 50.0;
            height += noise.get([global_x as f64 * 0.01, global_z as f64 * 0.01]) * 3.0;
            
            let height = height as i64;

            for height in 0..height {
                if height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64 && height >= key[1] * CHUNK_SIZE as i64 {
                    chunks.place([global_x, height, global_z], Block::Grass);
                }
            }
            let tree = noise.get([global_x as f64 * 2.13, global_z as f64 * 2.13]) > 0.485;
            if tree && height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64 && height >= key[1] * CHUNK_SIZE as i64 {
                let mirror_x = noise.get([global_x as f64 * 1.13, global_z as f64 * 1.13]) > 0.4325;
                let mirror_z = noise.get([global_x as f64 * 1.13, global_z as f64 * 1.13]) > 0.4750;
                chunks.place_structure(&flower_struct, [global_x, height, global_z], [mirror_x, false, mirror_z]);
            }
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

pub fn generate_flower() -> Structure {
    let mut grass = Structure::new([5, 5, 5]);

    let red = Block::Red;
    let green = Block::Green;
    let blue = Block::Blue;
    
    grass.place([1, 0, 2], green);
    grass.place([1, 1, 2], green);
    grass.place([2, 2, 2], green);

    grass.place([3, 2, 2], red);

    grass.place([3, 3, 2], blue);
    grass.place([3, 1, 2], blue);
    grass.place([3, 2, 1], blue);
    grass.place([3, 2, 3], blue);

    grass
}