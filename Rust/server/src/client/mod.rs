use std::net::SocketAddr;

use tokio::net::tcp::OwnedReadHalf;
use crate::channel::Sender;
use protocol::reader::Reader;
use protocol::event::Event;

use log::*;

// this function acts as kind of like a bridge between sync and async code
// the client part runs in the tokio runtime, to make many simultanious connectins possible
// all the handles run on seperate os threads, for performance predictability and ease of use reasons
pub async fn init(rw: (Reader<OwnedReadHalf>, Sender<Event>), addr: SocketAddr) {
    let mut reader = rw.0;
    let sender = rw.1;
    loop {
        if let Ok(event) = reader.get_event().await {
            match event {
                Event::ClientToServer(event) => {
                    use protocol::event::ClientToServer::*;
                    match event {
                        Register { name } => {
                            // todo!();
                        },
                        Login { token } => {
                            // todo!();
                        },
                        Move { coord } => {
                            // todo!();
                        },
                        PlaceStructure { pos, structure } => {
                            // todo!();
                        }
                    }
                }
                Event::ServerToClient(event) => {
                    warn!("{} sent an invalid event: {:?}", addr, event)
                }
                Event::Invalid => {
                    warn!("{} sent an invalid event", addr)
                }
            }
        } else {
            // client disconnected, preventing empty inv loop, important
            break
        }
    }
}