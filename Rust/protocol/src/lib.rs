pub mod reader;
pub mod writer;
pub mod error;
pub mod event;
pub mod chunk;
pub mod blocks;
pub mod channel;

const TCP_EVENT_BYTES: usize = 255;

pub type Token = [u8; 16];
pub type Coord = [i64; 3];
pub type PlayerCoord = [f64; 3];


pub fn calculate_chunk_distance(p1: &Coord, p2: &Coord) -> i64 {
    let distance = (( 
        (p1[0] - p2[0]).pow(2) +
        (p1[1] - p2[1]).pow(2) +
        (p1[2] - p2[2]).pow(2)
    ) as f64).sqrt();

    distance as i64
}

pub fn calculate_distance(p1: &PlayerCoord, p2: &PlayerCoord) -> f64 {
    let distance = (( 
        (p1[0] - p2[0]).powi(2) +
        (p1[1] - p2[1]).powi(2) +
        (p1[2] - p2[2]).powi(2)
    ) as f64).sqrt();

    distance
}

pub fn coord_to_player_coord(coord: Coord) -> PlayerCoord {
    [
        coord[0] as f64,
        coord[1] as f64,
        coord[2] as f64,
    ]
}

pub fn player_coord_to_coord(player_coord: PlayerCoord) -> Coord {
    [
        player_coord[0] as i64,
        player_coord[1] as i64,
        player_coord[2] as i64,
    ]
}

pub mod prelude {
    pub use super::{*, Token, Coord, PlayerCoord};
    pub use super::error::*;
    pub use super::event::prelude::*;
    pub use super::chunk::*;
    pub use super::blocks::Block;
    pub use super::channel::*;
}