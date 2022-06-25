use crate::{Token, Coord, PlayerCoord};
use crate::error::ClientError;
use serde::{Serialize, Deserialize};
use super::chunk::Structure;
use super::chunk::Chunk;
use super::chunk::ChunkDelta;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CTS {
    Register{name: String},
    Login{token: Token},
    Move{coord: PlayerCoord},
    PlaceStructure{coord: Coord, structure: Structure},
    Disconnect,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum STC {
    Error(ClientError),
    Token(Token),
    ChunkUnloads(Vec<Coord>),
    ChunkLoads(Vec<Chunk>),
    ChunkUpdates(Vec<ChunkDelta>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Invalid,
    CTS(CTS),
    STC(STC),
}

pub mod prelude {
    pub use super::CTS::{self, *};
    pub use super::STC::{self, *};
    pub use super::Event::*;
    pub use crate::error::ClientError;
}
