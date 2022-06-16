/* ONLY CODE IN THIS FILE IS ALLOWED TO RUN ASYNCHRONOUSLY */
/*     acts like a bridge between sync and async code      */
pub mod plugin;

use crate::channel::*;
use protocol::event::prelude::*;
use protocol::Token;
use std::mem::forget;

use protocol::{reader::Reader, writer::Writer};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::runtime;

pub struct Communicator {
    token: Token,
    event_sender: Sender<Box<ClientToServer>>,
    event_receiver: Receiver<Box<ServerToClient>>,
}
impl Communicator {
    pub fn init<H: ToSocketAddrs>(host: H) -> Option<Self> {
        let rt = runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
            .unwrap();
        
        if let Some((event_sender, event_receiver)) = rt.block_on(async move {
            if let Ok(socket) = TcpStream::connect(host).await {
                let (read, write) = socket.into_split();
                let mut reader = Reader::new(read);
                let mut writer = Writer::new(write);

                let (ev_sender_tx, ev_sender_rx): (Sender<Box<ClientToServer>>, Receiver<Box<ClientToServer>>) = bounded_channel(16);
                let (ev_receiver_tx, ev_receiver_rx) = bounded_channel(16);

                // INFO: tokio::spawn might be invalid in this context, but should be fine
                // rx
                tokio::spawn(async move {
                    loop {
                        if let Ok(event) = reader.get_event().await {
                            let event = event;
                            match event {
                                ServerToClient(event) => {
                                    if ev_receiver_tx.send(Box::new(event)).is_err() {
                                        log::warn!("failed to receive TCP event");
                                        break;
                                    }
                                },
                                ev => log::warn!("server sent invalid event: {:?}", ev),
                            }
                        } else {
                            log::info!("connection to host has been shut down");
                            break;
                        }
                    }
                });
                // tx
                tokio::spawn(async move {
                    loop {
                        if let Some(event) = ev_sender_rx.recv() {
                            if writer.send_event(&ClientToServer(*event)).await.is_err() {
                                log::warn!("failed to send TCP event");
                                break;
                            }
                        } else {
                            log::info!("TCP channel has been shut down");
                            break;
                        }
                    }
                });

                Some((ev_sender_tx, ev_receiver_rx))
            } else {
                None
            }
        }) {
            // not allowed to run destructor, must leak
            forget(rt);
            Some(Self { event_sender, event_receiver, token: [0u8; 16] })
        } else {
            None
        }
    }

    pub fn set_token(&mut self, token: Token) {
        self.token = token;
    }

    pub fn get_event(&self) -> Option<ServerToClient> {
        if let Some(event) = self.event_receiver.recv() {
            Some(*event)
        } else {
            None
        }
    }

    pub fn try_get_event(&self) -> Option<ServerToClient> {
        if let Some(event) = self.event_receiver.try_recv() {
            Some(*event)
        } else {
            None
        }
    }

    pub fn send_event(&self, event: ClientToServer) {
        if self.event_sender.send(Box::new(event)).is_err() {
            log::warn!("failed to send event");
        }
    }
}
