pub mod handle;
pub mod chunk_handle;

use protocol::{PlayerCoord, Token};

#[derive(Clone, Debug)]
pub enum Event {
    RequestChunks,
    Register{name: String},
    Login(Token),
    Logoff,
    Move(PlayerCoord),
}