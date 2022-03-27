
use protocol::{event::Event as TcpEvent, writer::Writer};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc;

pub fn init(
    mut tcp_receiver: mpsc::Receiver<TcpEvent>,
    mut writer: Writer<OwnedWriteHalf>,
) {
    tokio::spawn(async move {
        loop {
            let event = tcp_receiver.recv().await.unwrap();
            writer.send_event(&event).await.unwrap();
        }
    });
}