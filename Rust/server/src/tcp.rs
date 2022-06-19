use std::net::SocketAddr;

use tokio::io;
use tokio::net::{TcpStream, TcpListener, ToSocketAddrs};
use crate::queque::Queque;
use protocol::event::prelude::*;
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
    pub async fn accept(&self) -> io::Result<((Queque<ClientToServer>, Queque<ServerToClient>), SocketAddr)> {
        let (stream, addr) = self.listener.accept().await?;

        let (r, w) = TcpStream::into_split(stream);
        let mut reader = reader::Reader::new(r);
        let mut writer = writer::Writer::new(w);

        // INFO: channel has to be bounded, otherwise tokio's stack will overflow in debug mode
        let read_queque: Queque<ClientToServer> = Queque::new();
        let write_queque: Queque<ServerToClient> = Queque::new();

        // handles write operations to client
        let w_q = write_queque.clone();
        tokio::spawn(async move {
            // WARN!: THIS BLOCKING READ IS PROBABLY BAD INSIDE ASYNC CONTEXT
            while let Some(event) = w_q.recv() {
                writer.send_event(&ServerToClient(event)).await.unwrap();
            }
        });

        // handles read operations from client
        let r_q = read_queque.clone();
        let w_q = write_queque.clone();
        tokio::spawn(async move {
            while let Ok(event) = reader.get_event().await {
                match event {
                    ClientToServer(event) => {
                        // priotarisation
                        let important = match event {
                            ClientToServer::PlaceStructure{..} => true,
                            _ => false
                        };
                        if r_q.send(event, important).is_err() {
                            break
                        }
                    }
                    _ => {
                        log::warn!("{}, sent an invalid request, terminating connection ...", addr);
                        let _ = w_q.send(ServerToClient::Error(ClientError::ConnectionReset), true);
                        break;
                    }
                }
                // sleep(Duration::from_millis(10));
            }
            log::warn!("arstarsttsagjdr");
        });

        Ok(((read_queque, write_queque), addr))
    }
}