pub mod bridge;
pub mod reader;
pub mod writer;
pub mod event_queue;

use tokio::sync::mpsc;
use tokio::net::TcpStream;

use protocol::{reader::Reader as TcpReader, writer::Writer as TcpWriter};
use protocol::Token;

#[derive(Clone, Debug)]
pub enum GameEvent {
    Token(Token)
}

pub async fn init(stream: TcpStream) -> (bridge::Bridge, event_queue::Queue) {
    let (read, write) = stream.into_split();
    let reader = TcpReader::new(read);
    let writer = TcpWriter::new(write);

    let (bridge, tcp_rx, mut game_rx) = bridge::Bridge::init(); 
    reader::init(bridge.clone(), reader);
    writer::init(tcp_rx, writer);

    let event_queue = event_queue::Queue::init();

    // game of game_rx events to queue for game loop
    let eq_clone = event_queue.clone();
    tokio::spawn(async move {
        loop {
            let event = game_rx.recv().await.unwrap(); 
            eq_clone.send(event).await;
        }
    });

    (bridge, event_queue)
}