pub mod bridge;
pub mod reader;
pub mod writer;
pub mod event_queue;

use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use protocol::{reader::Reader as TcpReader, writer::Writer as TcpWriter};
use protocol::{Token, chunk::Chunk, error::Error};

pub struct Communicator {
    pub event_bridge: bridge::Bridge,
    pub event_queue: event_queue::Queue,
    _runtime: Runtime,
}
impl Communicator {
    pub fn init(ip: &str) -> Self {
        // init of client connection
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
            .unwrap();
    
        let (bridge, queue, game_rx) = rt.block_on(async move {
            let socket = TcpStream::connect(ip).await.unwrap();
            let (read, write) = socket.into_split();
            let reader = TcpReader::new(read);
            let writer = TcpWriter::new(write);
        
            let (bridge, tcp_rx, game_rx) = bridge::Bridge::init(); 
            reader::init(bridge.clone(), reader);
            writer::init(tcp_rx, writer);
        
            let event_queue = event_queue::Queue::init();
        
            return (bridge, event_queue, game_rx);
        });
    
        // game of game_rx events to queue for game loop
        let eq_clone = queue.clone();
        rt.spawn(async move {
            loop {
                let event = game_rx.recv().unwrap(); 
                eq_clone.send(event);
            }
        });
    
        Self {
            event_bridge: bridge,
            event_queue: queue,
            _runtime: rt,
        }
    }
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    Invalid,
    Token(Token),
    ChunkUpdate(Chunk),
    Error(Error),
}