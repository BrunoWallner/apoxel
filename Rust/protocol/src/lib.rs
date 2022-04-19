pub mod reader;
pub mod writer;
pub mod error;
pub mod event;
pub mod chunk;

pub type Token = [u8; 16];
pub type Coord = [i64; 3];
pub type PlayerCoord = [f64; 3];

pub const MAX_EVENTS_PER_SECOND: u64 = 20;

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