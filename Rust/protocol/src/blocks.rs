use serde::{Deserialize, Serialize};

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