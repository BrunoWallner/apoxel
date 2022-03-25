pub mod broadcaster;

use protocol::event::Event as ExternalEvent;
use protocol::Token;

#[derive(Clone, Debug)]
pub enum InternalEvent {
    SendToken(Token),
}

#[derive(Clone, Debug)]
pub enum Event {
    Internal(InternalEvent),
    External(ExternalEvent),
}