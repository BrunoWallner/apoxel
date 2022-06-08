mod noise;
mod terrain;
mod structures;

use protocol::chunk::Structure;

use protocol::chunk::Block;
use protocol::chunk::CHUNK_SIZE;
use protocol::chunk::{Chunk, SuperChunk};

const WATER_LEVEL: i64 = 20;

pub fn generate(chunk: Chunk, seed: u32) -> SuperChunk {
    let key = chunk.coord;
    let terrain = terrain::TerrainGen::new(seed);

    let tree_noise = noise::Noise::new(5874927);
    let stone_noise = noise::Noise::new(7826596);

    let noise = noise::Noise::new(86938);


    let mut chunks: SuperChunk = SuperChunk::new(chunk);
    /* Landscape */
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let global_x = (key[0] * CHUNK_SIZE as i64) + x as i64;
            let global_z = (key[2] * CHUNK_SIZE as i64) + z as i64;

            let (height, _biome) = terrain.get([global_x, global_z]);

            // dirt
            for height in 0..height {
                if height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64
                    && height >= key[1] * CHUNK_SIZE as i64
                {
                    chunks.place([global_x, height, global_z], Block::Dirt);
                }
            }

            // grass
            if height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64
                && height >= key[1] * CHUNK_SIZE as i64
            {
                let height_over_water = height - WATER_LEVEL + 1;
                if height_over_water <= 1 {
                    chunks.place([global_x, height, global_z], Block::Sand)
                }
                else {
                    chunks.place([global_x, height, global_z], Block::Grass)
                }
            }
            
            // trees
            let tree = tree_noise.get([global_x as f64, global_z as f64], 0.12, 1) > 0.51;
            if height > WATER_LEVEL && tree && height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64 && height >= key[1] * CHUNK_SIZE as i64 {
                let mirror_x = noise.get([global_x as f64, global_z as f64], 0.1, 1) > 0.4;
                chunks.place_structure(&structures::TREE, [global_x, height - 5, global_z], [mirror_x, false, false]);
            }
            // stones
            let stone = stone_noise.get([global_x as f64, global_z as f64], 0.12, 1) > 0.525;
            if stone && height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64 && height >= key[1] * CHUNK_SIZE as i64 {
                // let mirror_x = noise.get([global_x as f64 * 1.13, global_z as f64 * 1.13]) > 0.4325;
                // let mirror_z = noise.get([global_x as f64 * 1.13, global_z as f64 * 1.13]) > 0.4750;
                chunks.place_structure(&structures::STONE, [global_x, height - 2, global_z], [false, false, false]);
            }

            // water
            for height in 0..WATER_LEVEL {
                if height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64
                    && height >= key[1] * CHUNK_SIZE as i64
                {
                    let block = chunks.get([global_x, height, global_z]);
                    if block == Some(Block::None) || block == None {
                        chunks.place([global_x, height, global_z], Block::Water);
                    }
                }
            }
        }
    }

    // 3d
    // for x in 0..CHUNK_SIZE {
    //     for y in 0..CHUNK_SIZE {
    //         for z in 0..CHUNK_SIZE {
    //             let global_x = (key[0] * CHUNK_SIZE as i64) + x as i64;
    //             let global_y = (key[1] * CHUNK_SIZE as i64) + y as i64;
    //             let global_z = (key[2] * CHUNK_SIZE as i64) + z as i64;

    //             // let cave = noise.get([
    //             //     global_x as f64 * 0.022,
    //             //     global_y as f64 * 0.042,
    //             //     global_z as f64 * 0.022,
    //             // ]);
    //             // if cave > -0.0 {
    //             //     chunks.place([global_x, global_y, global_z], Block::Dirt);
    //             // }
    //             chunks.place([global_x, global_y, global_z], Block::Dirt);
    //         }
    //     }
    // }


    chunks
}

pub fn generate_flower() -> Structure {
    let mut grass = Structure::new([3, 3, 3]);

    let red = Block::Red;
    let green = Block::Green;
    let blue = Block::Blue;

    grass.place([1, 0, 1], green);
    grass.place([1, 1, 1], green);

    grass.place([1, 2, 1], red);

    grass.place([2, 2, 1], blue);
    grass.place([0, 2, 1], blue);
    grass.place([1, 2, 2], blue);
    grass.place([1, 2, 0], blue);

    grass
}
