mod noise;
mod structures;
mod terrain;

use protocol::chunk::Block;
use protocol::chunk::CHUNK_SIZE;
use protocol::chunk::{Chunk, SuperChunk};

const WATER_LEVEL: i64 = 20;

// pub fn generate(chunk: Chunk, _seed: u32) -> SuperChunk {
//     let noise = noise::Noise::new(86938);
//     let mut chunks: SuperChunk = SuperChunk::new(chunk);

//     for x in 0..CHUNK_SIZE {
//         for z in 0..CHUNK_SIZE {
//             let global_x = (chunk.coord[0] * CHUNK_SIZE as i64) + x as i64;
//             let global_z = (chunk.coord[2] * CHUNK_SIZE as i64) + z as i64;

//             let height = ((global_x as f32 * 0.05).sin() * 15.0) as i64;

//             if height < chunk.coord[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64
//                 && height >= chunk.coord[1] * CHUNK_SIZE as i64
//             {
//                 chunks.place([global_x, height, global_z], Block::Grass);

//                 // house
//                 let tree = noise.get([global_x as f64, global_z as f64], 0.12, 1, 150.0) > 0.5;
//                 if tree {
//                     chunks.place_structure(&structures::TREE, [global_x, height, global_z]);
//                 }
//             }
//         }
//     }

//     chunks
// }

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

            if height < key[1] * CHUNK_SIZE as i64 + CHUNK_SIZE as i64
                && height >= key[1] * CHUNK_SIZE as i64
            {
                for h in 0..height {
                    let block = match h {
                        // Sand
                        h if h < WATER_LEVEL + 3
                            && chunks
                                .get([global_x, h + 1, global_z])
                                .unwrap_or(Block::None)
                                .to_category()
                                .0
                                == 0 =>
                        {
                            Block::Sand
                        }
                        // Grass
                        h if h > height - 4 => Block::Grass,
                        _ => Block::Dirt,
                    };
                    chunks.place([global_x, h, global_z], block);

                    // Water
                    for h in height..WATER_LEVEL {
                        let coord = [global_x, h, global_z];
                        if chunks.get(coord).unwrap_or(Block::None).to_category().0 == 0 {
                            chunks.place(coord, Block::Water);
                        }
                    }
                }

                // trees
                let tree = tree_noise.get([global_x as f64, global_z as f64], 0.12, 1, 30.0) > 0.5;
                if tree && height > WATER_LEVEL {
                    chunks.place_structure(&structures::TREE, [global_x, height - 5, global_z]);
                }
                // stones
                let stone =
                    stone_noise.get([global_x as f64, global_z as f64], 0.12, 1, 100.0) > 0.53;
                if stone {
                    chunks.place_structure(&structures::STONE, [global_x, height - 2, global_z]);
                }
                // house
                let house =
                    stone_noise.get([global_x as f64, global_z as f64], 0.12, 1, 150.0) > 0.54;
                if house && height > WATER_LEVEL {
                    chunks.place_structure(&structures::HOUSE, [global_x, height - 7, global_z]);
                }
            }
        }
    }
    chunks
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
