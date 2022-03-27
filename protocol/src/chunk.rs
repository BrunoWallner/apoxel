use serde::{Serialize, Deserialize};
use super::Coord;
use std::collections::HashMap;

pub const CHUNK_SIZE: usize = 32;

pub fn get_chunk_coords(coord: &[i64; 3]) -> ([i64; 3], [usize; 3]) {
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

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct SuperChunk {
    pub chunks: HashMap<[i64; 3], Chunk>,
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

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum Block {
    None,
    Air,

    Grass,
    Dirt,

    Stone,
    DarkStone,

    Leave,
}
impl Block {
    pub fn to_category(&self) -> (u32, u32) {
        match self {
            Block::None => (0, 0),
            Block::Air => (0, 1),

            Block::Grass => (1, 0),
            Block::Dirt => (1, 1),

            Block::Stone => (2, 0),
            Block::DarkStone => (2, 1),

            Block::Leave => (3, 0),
        }
    }

    pub fn is_transparent(&self) -> bool {
        match self {
            Block::None => true,
            Block::Air => true,
            _ => false,
        }
    }

    pub fn is_semi_transparent(&self) -> bool {
        match self {
            Block::Leave => true,
            _ => false,
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

pub fn generate_tree() -> Structure {
    let mut tree = Structure::new([20, 25, 20]);

    let rad: i32 = 9;
    for x in -rad..rad {
        for y in -rad..rad {
            for z in -rad..rad {
                if x.pow(2) + z.pow(2) + y.pow(2) >= rad.pow(2) {
                    continue;
                }
                tree.place([x as usize + 10, y as usize + 15, z as usize + 10], Block::Leave);
            }
        }
    }

    for y in 0..23 {
        tree.place([10, y, 10], Block::Dirt);
    }

    tree
}
