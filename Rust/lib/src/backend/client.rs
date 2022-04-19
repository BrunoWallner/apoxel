use super::bridge::Bridge;

use crossbeam::channel;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use protocol::event::Event;
use protocol::{reader::Reader as TcpReader, writer::Writer as TcpWriter};

use gdnative::prelude::*;

use crate::terminator::Terminator;


pub fn init(host: String, runtime: &Runtime, terminator: Terminator) -> Option<Bridge> {
    if let Some((mut reader, mut writer)) = runtime.block_on(async move {
        let socket = TcpStream::connect(host.clone()).await;
        match socket {
            Ok(socket) => {
                let (read, write) = socket.into_split();
                let reader = TcpReader::new(read);
                let writer = TcpWriter::new(write);
        
                Some( (reader, writer) )
            }
            Err(_) => {
                None
            }
        }
    }) {
        let bridge = runtime.block_on(async move {
            let (out_tx, out_rx) = channel::unbounded();
    
            // init of output
            let output = out_rx;
            let term = terminator.clone();
            tokio::spawn(async move {
                loop {
                    if term.should_terminate() {
                        drop(writer);
                        break;
                    }
                    if let Ok(ev) = output.recv() {
                        let _ = writer.send_event(&Event::ClientToServer(ev)).await;
                    }
                }
            });
    
            // init of input
            let (in_tx, in_rx) = channel::unbounded();
            let input = in_tx;
            tokio::spawn(async move {
                loop {
                    if terminator.should_terminate() {
                        drop(reader);
                        drop(input);
                        break;
                    }
                    if let Ok(event) = reader.get_event().await {
                        match event {
                            Event::ServerToClient(ev) => {
                                let _ = input.send(ev);
                            }
                            Event::ClientToServer(_) | Event::Invalid => {
                                godot_print!("received invalid event from server (client.rs)");
                            }
                        }
                    }
                }
            });
    
            Bridge {
                event_receiver: in_rx,
                event_sender: out_tx,
            }
        });
        Some(bridge)   
    } else {
        None
    }
}