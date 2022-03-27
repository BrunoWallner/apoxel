use crate::{Token, Coord, PlayerCoord};
use crate::error::Error;
use serde::{Serialize, Deserialize};
use super::chunk::Structure;
use super::chunk::Chunk;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Invalid,
    Error(Error),
    Token(Token),
    Register{name: String},
    Login{token: Token},
    MovePlayer(PlayerCoord),
    ChunkUpdate(Chunk),
    PlaceStructure{pos: Coord, structure: Structure},
}