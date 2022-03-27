pub mod reader;
pub mod writer;
pub mod error;
pub mod event;
pub mod chunk;

pub type Token = [u8; 16];
pub type Coord = [i64; 3];
pub type PlayerCoord = [f64; 3];