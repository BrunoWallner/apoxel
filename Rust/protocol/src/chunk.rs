use super::Coord;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const CHUNK_SIZE: usize = 32;
pub type ChunkData = Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>;

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
            data: Box::new([[[Block::None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
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
    // pub fn place_structure(&mut self, structure: &Structure, coord: [i64; 3], mirror: [bool; 3]) {
    //     // to center structure on x and z
    //     let x_offset = structure.size[0] / 2;
    //     let z_offset = structure.size[2] / 2;

    //     for x in 0..structure.size[0] {
    //         for y in 0..structure.size[1] {
    //             for z in 0..structure.size[2] {
    //                 let block = structure.get([x, y, z]).unwrap();
    //                 if block != Block::None {
    //                     let mut x = x;
    //                     let mut y = y;
    //                     let mut z = z;
    //                     // mirroring
    //                     if mirror[0] {
    //                         x = structure.size[0] - x
    //                     }
    //                     if mirror[1] {
    //                         y = structure.size[1] - y
    //                     }
    //                     if mirror[2] {
    //                         z = structure.size[2] - z
    //                     }

    //                     let x = coord[0] - x_offset as i64 + x as i64;
    //                     let y = coord[1] + y as i64;
    //                     let z = coord[2] - z_offset as i64 + z as i64;

    //                     self.place([x, y, z], block);
    //                 }
    //             }
    //         }
    //     }
    // }

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

const BLOCK_COLORS: [[u8; 3]; 14] = [
    [0, 0, 0],
    [255, 255, 255],
    // terrain
    [10, 200, 30],
    [50, 30, 30],
    [100, 100, 100],
    [70, 70, 70],
    [190, 160, 60],
    // woods
    [75, 35, 35],
    [115, 65, 35],
    // colors
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
    // foliage
    [40, 110, 40],
    // liquids
    [0, 100, 200],
];

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Block {
    None,
    Air,

    // terrain
    Grass,
    Dirt,

    Stone,
    DarkStone,

    Sand,

    // woods
    OakWood,
    AppleTreeWood,

    // colors
    Red,
    Green,
    Blue,

    // foliage
    OakLeave,

    // liquids
    Water,
}

impl Block {
    pub fn to_category(&self) -> (u8, u8) {
        match self {
            Block::None => (0, 0),
            Block::Air => (0, 1),

            Block::Grass => (1, 0),
            Block::Dirt => (1, 1),

            Block::Stone => (2, 0),
            Block::DarkStone => (2, 1),

            Block::Sand => (3, 2),

            Block::OakWood => (4, 0),
            Block::AppleTreeWood => (4, 1),

            Block::Red => (5, 0),
            Block::Green => (5, 1),
            Block::Blue => (5, 2),

            Block::OakLeave => (6, 0),

            Block::Water => (7, 0),
        }
    }

    pub fn transparency(&self) -> Option<f32> {
        match self {
            Block::None => Some(0.0),
            Block::Air => Some(0.0),
            Block::Water => Some(0.5),
            _ => None,
        }
    }

    pub fn from_color(color: [u8; 3]) -> Self {
        // WARN! MUST BE UPDATED
        let first = Block::None as u8;
        let last = Block::Water as u8;

        if let Some(position) = BLOCK_COLORS.iter().position(|x| x == &color) {
            match position as u8 {
                // Block::Water as last Block in Block enum
                i if i >= first && i <= last =>
                // WARN! This IS VERY UNSAFE, first and last MUST BE CORRECT!
                unsafe { std::mem::transmute(i) },
                _ => Block::None,
            }
        } else {
            Block::None
        }
    }

    pub fn to_color(&self) -> [u8; 3] {
        let index = *self as u16;
        BLOCK_COLORS[index as usize]
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
