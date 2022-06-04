use protocol::event::Event;
//use protocol::{Token, Coord, chunk::Chunk};

// instructions for communication between tcp reader and writer
#[derive(Clone, Debug)]
pub enum Tcp {
    Protocol(Event),
}