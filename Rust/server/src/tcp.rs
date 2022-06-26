use std::net::SocketAddr;

use tokio::io;
use tokio::net::{TcpStream, TcpListener, ToSocketAddrs};
use protocol::prelude::*;
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
    pub async fn accept(&self) -> io::Result<((Sender<STC>, Receiver<CTS>), SocketAddr)> {
        let (stream, addr) = self.listener.accept().await?;

        let (r, w) = TcpStream::into_split(stream);
        let mut reader = reader::Reader::new(r);
        let mut writer = writer::Writer::new(w);

        // POV: server to remote client
        let (sender_tx, sender_rx) = channel(Some(64));
        let (receiver_tx, receiver_rx) = channel(Some(64));

        // handles write operations to client
        let rx = sender_rx.clone();
        tokio::spawn(async move {
            // WARN!: THIS BLOCKING READ IS PROBABLY BAD INSIDE ASYNC CONTEXT
            while let Ok(event) = rx.recv() {
                writer.send_event(&STC(event)).await.unwrap();
            }
        });

        // handles read operations from client
        let tx = receiver_tx.clone();
        let client_tx = sender_tx.clone();
        tokio::spawn(async move {
            while let Ok(event) = reader.get_event().await {
                match event {
                    CTS(event) => {
                        // priotarisation
                        let important = match event {
                            CTS::PlaceStructure{..} => true,
                            _ => false
                        };
                        if tx.send(event, important).is_err() {
                            break
                        }
                    }
                    _ => {
                        log::warn!("{}, sent an invalid TCP request, terminating connection ...", addr);
                        let _ = client_tx.send(STC::Error(ClientError::ConnectionReset), true);
                        break;
                    }
                }
            }
        });

        Ok(((sender_tx, receiver_rx), addr))
    }
}