pub mod generation;
pub mod blocks;
pub mod structure;
pub mod handle;

pub const CHUNK_SIZE: usize = 32;

use blocks::Block;
use protocol::Coord;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Chunk {
    pub data: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub coord: Coord,
}
impl Chunk {
    pub fn new(coord: Coord) -> Self {
        Self {
            data: [[[Block::None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            coord,
        }
    }
    pub fn merge(&mut self, other: &Self) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if other.data[x][y][z] != Block::None {
                        self.data[x][y][z] = other.data[x][y][z];
                    }
                }
            }

        }
    }
}

fn get_chunk_coords(coord: &[i64; 3]) -> ([i64; 3], [usize; 3]) {
    let chunk = [
        (coord[0] as f64 / CHUNK_SIZE as f64).floor() as i64,
        (coord[1] as f64 / CHUNK_SIZE as f64).floor() as i64,
        (coord[2] as f64 / CHUNK_SIZE as f64).floor() as i64
    ];
    let index = [
        (coord[0] - (chunk[0] * CHUNK_SIZE as i64)).abs() as usize,
        (coord[1] - (chunk[1] * CHUNK_SIZE as i64)).abs() as usize,
        (coord[2] - (chunk[2] * CHUNK_SIZE as i64)).abs() as usize
    ];

    (chunk, index)
}

use std::collections::HashMap;
use structure::Structure;

pub struct SuperChunk {
    chunks: HashMap<[i64; 3], Chunk>,
} impl SuperChunk {
    pub fn new(main: ([i64; 3], Chunk)) -> Self {
        let mut chunks = HashMap::default();
        chunks.insert(main.0, main.1);

        Self {
            chunks
        }
    }
    pub fn get(&self, coord: [i64; 3]) -> Option<Block> {
        let (chunk, index) = get_chunk_coords(&coord);
    
        if let Some(c) = self.chunks.get(&chunk) {
            return Some(c.data[index[0]][index[1]][index[2]])
        } else {
            return None
        } 
    }
    pub fn place(&mut self, coord: [i64; 3], block: Block) {
        if block != Block::None {
            let (chunk, index) = get_chunk_coords(&coord);
    
            if let Some(c) = self.chunks.get_mut(&chunk) {
                c.data[index[0]][index[1]][index[2]] = block;
            } else {
                let mut c = Chunk::new(chunk);
                c.data[index[0]][index[1]][index[2]] = block;
                self.chunks.insert(chunk, c);
            }
        }
    }
    pub fn place_structure(&mut self, structure: &Structure, coord: [i64; 3]) {
        // to center structure on x and z
        let x_offset = structure.size[0] / 2;
        let z_offset = structure.size[2] / 2;

        for x in 0..structure.size[0] {
            for y in 0..structure.size[1] {
                for z in 0..structure.size[2] {
                    let block = structure.get([x, y, z]).unwrap();
                    if block != Block::None {
                        let x = coord[0] - x_offset as i64 + x as i64;
                        let y = coord[1] + y as i64;
                        let z = coord[2] - z_offset as i64 + z as i64;
    
                        self.place([x, y, z], block);
                    }
                }
            }
        }
    }
}