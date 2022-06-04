pub mod chunk_handle;
pub mod handle;

use crate::player::Player;
use protocol::{PlayerCoord, Token};
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum Event {
    RequestChunks,
    Register { name: String },
    Login(Token),
    Logoff,
    Move(PlayerCoord),
    GetPlayer(mpsc::Sender<Option<Player>>),
}
