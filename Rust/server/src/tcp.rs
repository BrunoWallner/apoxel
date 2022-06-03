use tokio::io;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpStream, TcpListener, ToSocketAddrs};
use crate::channel;
use protocol::event::Event;
use protocol::{reader, writer};

use log::info;

pub struct Tcp {
    listener: TcpListener,
}
impl Tcp {
    pub async fn init<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self{listener})
    }

    pub async fn accept(&self) -> io::Result<(reader::Reader<OwnedReadHalf>, channel::Sender<Event>)> {
        let (stream, info) = self.listener.accept().await?;
        info!("new connection: {}", info);

        let (r, w) = TcpStream::into_split(stream);
        let reader = reader::Reader::new(r);
        let mut writer = writer::Writer::new(w);

        let (sender, receiver): (channel::Sender<Event>, channel::Receiver<Event>) = channel::new();
        tokio::spawn(async move {
            loop {
                if let Some(event) = receiver.recv() {
                    writer.send_event(&event).await.unwrap();
                }
            }
        });

        Ok((reader, sender))
    }
}