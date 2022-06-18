use std::net::SocketAddr;

use tokio::io;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpStream, TcpListener, ToSocketAddrs};
use crate::channel::*;
use protocol::event::Event;
use protocol::{reader, writer};

pub struct Tcp {
    listener: TcpListener,
}
impl Tcp {
    pub async fn init<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self{listener})
    }

    // blockingly accept new client
    pub async fn accept(&self) -> io::Result<((reader::Reader<OwnedReadHalf>, Sender<Event>), SocketAddr)> {
        let (stream, addr) = self.listener.accept().await?;

        let (r, w) = TcpStream::into_split(stream);
        let reader = reader::Reader::new(r);
        let mut writer = writer::Writer::new(w);

        // INFO: channel has to be bounded, otherwise tokio's stack will overflow in debug mode
        let (sender, receiver): (Sender<Event>, Receiver<Event>) = channel();
        tokio::spawn(async move {
            // WARN: I THINK THE RANDOM UNRECOVERABLE HANG UPS ARE CAUSED BY ANY OF THIS
            while let Some(event) = receiver.recv() {
                let ev = &format!("{:?}", event);
                let has_len = ev.len() > 50;
                log::info!("sent event: {}", if has_len {&ev[0..50]} else {ev});
                writer.send_event(&event).await.unwrap();
            }
        });

        Ok(((reader, sender), addr))
    }
}