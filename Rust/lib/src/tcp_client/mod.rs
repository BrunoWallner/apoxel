pub mod client;
pub mod bridge;

use bridge::Bridge;
use protocol::event::{ClientToServer, ServerToClient};
use protocol::chunk::CHUNK_SIZE;

use crate::Coord;

use tokio::runtime::Runtime;
use gdnative::prelude::*;

use std::thread;
use crossbeam::channel;

const MAX_CHUNK_UPDATES_PER_CYCLE: usize = 10;
const MAX_EVENTS_PER_CYCLE: usize = 10;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TcpClient {
    bridge: Option<Bridge>,
    runtime: Runtime,

    events: Vec<(String, Vec<Variant>)>,
    //chunk_updates: Vec<(Coord, ByteArray)>,

    chunk_update_sender: channel::Sender<(Coord, ByteArray)>,
    chunk_update_receiver: channel::Receiver<(Coord, ByteArray)>,
}

#[methods]
impl TcpClient {
    fn new(_owner: &Node) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
            .unwrap();

        let (chunk_update_sender, chunk_update_receiver) = channel::unbounded();

        TcpClient { 
            bridge: None,
            runtime,

            events: Vec::new(),
            
            chunk_update_sender,
            chunk_update_receiver,
        }
    }

    #[export]
    fn establish_connection(&mut self, _owner: &Node, host: String) -> bool {
        if let Some( bridge ) = client::init(host, &self.runtime) {
            self.bridge = Some(bridge);
            return true;
        } else {
            return false;
        }
    }

    #[export]
    fn connection_established(&self, _owner: &Node) -> bool {
        self.bridge.is_some()
    }

    #[export]
    fn fetch_event(&mut self, _owner: &Node) -> Option<(String, Vec<Variant>)> {
        self.events.pop()
    }

    #[export]
    fn fetch_chunk_update(&mut self, _owner: &Node) -> Vec<(Coord, ByteArray)> {
        // if self.chunk_updates.len() > MAX_CHUNK_UPDATES_PER_CYCLE {
        //     self.chunk_updates.drain(..MAX_CHUNK_UPDATES_PER_CYCLE).as_slice().to_vec()
        // } else {
        //     self.chunk_updates.drain(..).as_slice().to_vec()
        // }
        let mut events: Vec<(Coord, ByteArray)> = Vec::new();
        for _ in 0..MAX_CHUNK_UPDATES_PER_CYCLE {
            if let Ok(event) = self.chunk_update_receiver.try_recv() {
                events.push(event)
            } else {
                break;
            }
        }
        events
    }

    #[export]
    fn send(&self, _owner: &Node, event: String) -> bool {
        if let Some(bridge) = &self.bridge {
            let ev: Result<ClientToServer, serde_json::Error> = serde_json::from_str(&event);

            match ev {
                Ok(event) => {
                    bridge.send(event.clone()); 
                }
                Err(e) => {
                    godot_print!("attempted to send invalid tcp event:\n{}\n{:?}", e, event);
                }
            }
            return true;
        } else {
            return false;
        }
    }

    #[export]
    fn _process(&mut self, _owner: &Node, _dt: f64) {
        let now = std::time::Instant::now();

        if let Some(bridge) = &self.bridge {
            for _ in 0..MAX_EVENTS_PER_CYCLE {
                if let Some(event) = bridge.receive() {
                    match event {
                        ServerToClient::ChunkUpdate(chunk) => {
                            let sender = self.chunk_update_sender.clone();
                            thread::spawn(move || {
                                let now = std::time::Instant::now();

                                let mut coord: PoolArray<i32> = PoolArray::new();
                                for c in chunk.coord.iter() {
                                    coord.push(*c as i32 * CHUNK_SIZE as i32);
                                }
    
                                let mut data: ByteArray = PoolArray::from_vec(vec![0; CHUNK_SIZE.pow(3) * 2]);
                                for x in 0..CHUNK_SIZE {
                                    for y in 0..CHUNK_SIZE {
                                        for z in 0..CHUNK_SIZE {
                                            // https://coderwall.com/p/fzni3g/bidirectional-translation-between-1d-and-3d-arrays
                                            let i = (x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE) * 2;
                                            data.set(i as i32, chunk.data[x][y][z].to_category().0);
                                            data.set(i as i32 + 1, chunk.data[x][y][z].to_category().1);
                                        }
                                    }
                                }
                                sender.send( (coord, data) ).unwrap();
                            });
                        }
                        ServerToClient::Token(token) => {
                            self.events.push( (String::from("Token"), vec![Variant::new(token.to_vec())]) );
                        }
                        ServerToClient::Error(error) => {
                            self.events.push( (String::from("Error"), vec![Variant::new(format!("{:?}", error))]) );
                        }
                    }
                }
            }
        }

        //godot_print!("_process inf tcp_client took: {} µs", now.elapsed().as_micros());
    }
}