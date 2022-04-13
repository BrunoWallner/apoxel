use super::bridge::Bridge;

use crossbeam::channel;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use protocol::event::Event;
use protocol::{reader::Reader as TcpReader, writer::Writer as TcpWriter};

use gdnative::prelude::*;


pub fn init(host: String, runtime: &Runtime) -> Option<Bridge> {
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
            let output = out_rx.clone();
            tokio::spawn(async move {
                loop {
                    let ev = output.recv().unwrap();
                    writer.send_event(&Event::ClientToServer(ev)).await.unwrap();
                }
            });
    
            // init of input
            let (in_tx, in_rx) = channel::unbounded();
            let input = in_tx.clone();
            tokio::spawn(async move {
                loop {
                    let event = reader.get_event().await.unwrap();
                    match event {
                        Event::ServerToClient(ev) => {
                            input.send(ev).unwrap();
                        }
                        Event::ClientToServer(_) | Event::Invalid => {
                            godot_print!("received invalid event from server (client.rs)");
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