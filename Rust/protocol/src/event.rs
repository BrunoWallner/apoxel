use crate::{Token, Coord, PlayerCoord};
use crate::error::ClientError;
use serde::{Serialize, Deserialize};
use super::chunk::Structure;
use super::chunk::Chunk;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientToServer {
    Register{name: String},
    Login{token: Token},
    Move{coord: PlayerCoord},
    PlaceStructure{pos: Coord, structure: Structure},
    Disconnect,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerToClient {
    Error(ClientError),
    Token(Token),
    ChunkUpdate(Chunk),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Invalid,
    ClientToServer(ClientToServer),
    ServerToClient(ServerToClient),
}

pub mod prelude {
    pub use super::ClientToServer::{self, *};
    pub use super::ServerToClient::{self, *};
    pub use super::Event::*;
    pub use crate::error::ClientError;
}
