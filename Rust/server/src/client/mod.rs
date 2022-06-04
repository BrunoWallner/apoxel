use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use crate::channel::Sender;
use protocol::reader::Reader;
use protocol::event::Event;

use log::*;

pub async fn init(rw: (Reader<OwnedReadHalf>, Sender<Event>)) {
    let mut reader = rw.0;
    let sender = rw.1;
    loop {
        if let Ok(event) = reader.get_event().await {
            info!("got event: {:#?}", event);
        } else {
            // client disconnected, preventing empty inv loop
            break
        }
    }
}