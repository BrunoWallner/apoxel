use super::Coord;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use super::blocks::Block;

// bounded by `ChunkIndex`
// currently has to have u8 size limit
// I just dont want to use `as usize` as much
pub const CHUNK_SIZE: usize = 64;

// pub type ChunkData = Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>;
pub type ChunkData = Vec<Vec<Vec<Block>>>;


// to save memory in ChunkDelta
// if CHUNK_SIZE is bigger than 255 this must be at least 
pub type ChunkIndex = [u8; 3];
pub type ChunkDelta = (Coord, Vec<(ChunkIndex, Block)>);

pub fn get_chunk_coords(coord: &[i64; 3]) -> ([i64; 3], [usize; 3]) {
    // Must work
    let chunk = [
        (coord[0] as f64 / CHUNK_SIZE as f64).floor() as i64,
        (coord[1] as f64 / CHUNK_SIZE as f64).floor() as i64,
        (coord[2] as f64 / CHUNK_SIZE as f64).floor() as i64,
    ];
    let index = [
        (coord[0] - (chunk[0] * CHUNK_SIZE as i64)).abs() as usize,
        (coord[1] - (chunk[1] * CHUNK_SIZE as i64)).abs() as usize,
        (coord[2] - (chunk[2] * CHUNK_SIZE as i64)).abs() as usize,
    ];

    (chunk, index)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub data: ChunkData,
    pub coord: Coord,
}
impl Chunk {
    pub fn new(coord: Coord) -> Self {
        Self {
            data: vec![vec![vec![Block::None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
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
    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.data[x][y][z] != Block::None {
                        empty = false;
                    }
                }
            }
        }
        empty
    }
    pub fn get_delta(&self, other: &Self) -> ChunkDelta {
        let mut delta: Vec<(ChunkIndex, Block)> = Vec::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.data[x][y][z] != other.data[x][y][z] {
                        // WARN: only OK when CHUNK_SIZE <= 255
                        delta.push(([x as u8, y as u8, z as u8], other.data[x][y][z]));
                    }
                }
            }
        }
        (self.coord, delta)
    }
    pub fn apply_delta(&mut self, delta: &ChunkDelta) {
        // INFO: might want to check if block is None
        for (coord, block) in delta.1.iter() {
            self.data[coord[0] as usize][coord[1] as usize][coord[2] as usize] = *block;
        }
    }
    // SIDES
    pub fn get_left_side(&self) -> [[Block; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut side = [[Block::None; CHUNK_SIZE]; CHUNK_SIZE];
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                side[z][y] = self.data[0][y][z]
            }
        }
        side
    }
    pub fn get_right_side(&self) -> [[Block; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut side = [[Block::None; CHUNK_SIZE]; CHUNK_SIZE];
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                side[z][y] = self.data[CHUNK_SIZE - 1][y][z]
            }
        }
        side
    }
    pub fn get_front_side(&self) -> [[Block; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut side = [[Block::None; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                side[x][y] = self.data[x][y][CHUNK_SIZE - 1]
            }
        }
        side
    }
    pub fn get_back_side(&self) -> [[Block; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut side = [[Block::None; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                side[x][y] = self.data[x][y][0]
            }
        }
        side
    }
    pub fn get_top_side(&self) -> [[Block; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut side = [[Block::None; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                side[x][z] = self.data[x][CHUNK_SIZE - 1][z]
            }
        }
        side
    }
    pub fn get_bottom_side(&self) -> [[Block; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut side = [[Block::None; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                side[x][z] = self.data[x][0][z]
            }
        }
        side
    }
}

#[derive(Clone, Debug)]
pub struct SuperChunk {
    pub chunks: BTreeMap<[i64; 3], Chunk>,
    pub main_chunk: Coord,
}
impl SuperChunk {
    pub fn new(main_chunk: Chunk) -> Self {
        let mut chunks = BTreeMap::default();
        chunks.insert(main_chunk.coord, main_chunk.clone());

        Self {
            chunks,
            main_chunk: main_chunk.coord,
        }
    }
    pub fn get_main_chunk(&self) -> Chunk {
        // guaranteed to not panic if initialized with Self::new()
        self.chunks.get(&self.main_chunk).unwrap().clone()
    }
    pub fn remove_main_chunk(&mut self) -> Chunk {
        // guaranteed to not panic if initialized with Self::new()
        self.chunks.remove(&self.main_chunk).unwrap()
    }
    pub fn get(&self, coord: [i64; 3]) -> Option<Block> {
        let (chunk, index) = get_chunk_coords(&coord);

        if let Some(c) = self.chunks.get(&chunk) {
            return Some(c.data[index[0]][index[1]][index[2]]);
        } else {
            return None;
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

    // old
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Structure {
    pub size: [usize; 3],
    pub voxels: Vec<Vec<Vec<Block>>>,
}
impl Structure {
    pub fn new(size: [usize; 3]) -> Self {
        Self {
            size,
            voxels: vec![vec![vec![Block::None; size[2]]; size[1]]; size[0]],
        }
    }

    pub fn place(&mut self, coord: [usize; 3], block: Block) -> bool {
        if coord[0] < self.size[0] && coord[1] < self.size[1] && coord[2] < self.size[2] {
            self.voxels[coord[0]][coord[1]][coord[2]] = block;
        } else {
            return false;
        }
        return true;
    }

    pub fn get(&self, coord: [usize; 3]) -> Option<Block> {
        if coord[0] < self.size[0] && coord[1] < self.size[1] && coord[2] < self.size[2] {
            return Some(self.voxels[coord[0]][coord[1]][coord[2]]);
        } else {
            return None;
        }
    }
}
