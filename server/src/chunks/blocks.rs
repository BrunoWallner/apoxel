#![allow(dead_code)]

#[derive(Copy, Clone, Debug, PartialEq)]
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