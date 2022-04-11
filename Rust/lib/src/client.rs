use crossbeam::channel;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use std::sync::{Arc, Mutex};

use protocol::event::Event;
use protocol::event::ServerToClient;
use protocol::event::ClientToServer;
use protocol::{reader::Reader as TcpReader, writer::Writer as TcpWriter};

use gdnative::prelude::*;

#[derive(Clone, Debug)]
pub struct Bridge {
    input: Arc<Mutex<Vec<ServerToClient>>>,
    output: channel::Sender<ClientToServer>,
}
impl Bridge {
    pub fn receive(&self) -> Option<ServerToClient> {
        let mut input = self.input.lock().unwrap();
        input.pop()
    }
    pub fn send(&self, event: ClientToServer) {
        self.output.send(event).unwrap();
    }
}

pub fn init(host: String) -> (Runtime, Bridge) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .unwrap();

    let (mut reader, mut writer) = rt.block_on(async move {
        let socket = TcpStream::connect(host.clone()).await.unwrap();
        godot_print!("connected to: {}", host);
        let (read, write) = socket.into_split();
        let reader = TcpReader::new(read);
        let writer = TcpWriter::new(write);

        (reader, writer)
    });

    let bridge = rt.block_on(async move {
        let (tx, rx) = channel::unbounded();

        // init of output
        let output = rx.clone();
        tokio::spawn(async move {
            loop {
                let ev = output.recv().unwrap();
                writer.send_event(&Event::ClientToServer(ev)).await.unwrap();
            }
        });

        // init of input
        let input = Arc::new(Mutex::new(Vec::new()));
        let input_clone = input.clone();
        tokio::spawn(async move {
            loop {
                let event = reader.get_event().await.unwrap();
                match event {
                    Event::ServerToClient(ev) => {
                        let mut input = input_clone.lock().unwrap();
                        input.push(ev);
                    }
                    Event::ClientToServer(_) | Event::Invalid => {
                        godot_print!("invalid event from server (client.rs)");
                    }
                }
            }
        });

        Bridge {
            input,
            output: tx,
        }
    });

    (rt, bridge)
}