pub mod bridge;
pub mod reader;
pub mod writer;
pub mod game_event_queue;

use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use protocol::{reader::Reader as TcpReader, writer::Writer as TcpWriter};
use protocol::{Token, chunk::Chunk, error::Error};

use crossbeam::channel;

// make sure not to clone game_rx and thus handle events multiple times
pub struct Communicator {
    pub event_bridge: bridge::Bridge,
    //pub event_queue: game_event_queue::GameEventQueue,
    pub communication_rx: channel::Receiver<CommunicationEvent>,
    _runtime: Runtime, // must stay in scope
}
impl Communicator {
    pub fn init(ip: &str) -> Self {
        // init of client connection
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
            .unwrap();
    
        let (bridge, communication_rx) = rt.block_on(async move {
            let socket = TcpStream::connect(ip).await.unwrap();
            let (read, write) = socket.into_split();
            let reader = TcpReader::new(read);
            let writer = TcpWriter::new(write);
        
            let (bridge, tcp_rx, communication_rx) = bridge::Bridge::init(); 
            reader::init(bridge.clone(), reader);
            writer::init(tcp_rx, writer);
        
            //let geq = game_event_queue::GameEventQueue::init();
        
            return (bridge, communication_rx);
        });
    
        /* WOULD CURRENTLY ONLY RESULT IN OVERHEAD
        // fetch all game events using game_rx to push it to GameEventQueue
        // to be able to pull these events non blocking in game loop
        let geq_clone = geq.clone();
        rt.spawn(async move {
            loop {
                let event = game_rx.recv().unwrap(); 
                geq_clone.send(event);
            }
        });
        */
    
        Self {
            event_bridge: bridge,
            communication_rx,
            _runtime: rt,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CommunicationEvent {
    Invalid,
    Token(Token),
    ChunkUpdate(Chunk),
    Error(Error),
}