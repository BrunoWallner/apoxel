use protocol::event::Event;

// instructions for communication between tcp reader and writer
#[derive(Clone, Debug)]
pub enum Tcp {
    Protocol(Event),
}