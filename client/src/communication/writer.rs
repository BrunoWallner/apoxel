
use protocol::{event::Event as TcpEvent, writer::Writer};
use tokio::net::tcp::OwnedWriteHalf;

use crossbeam::channel;

pub fn init(
    tcp_receiver: channel::Receiver<TcpEvent>,
    mut writer: Writer<OwnedWriteHalf>,
) {
    tokio::spawn(async move {
        loop {
            let event = tcp_receiver.recv().unwrap();
            writer.send_event(&event).await.unwrap();
        }
    });
}