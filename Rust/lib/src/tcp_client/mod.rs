pub mod client;
pub mod bridge;

use bridge::Bridge;
use protocol::event::{ClientToServer, ServerToClient};
use protocol::chunk::CHUNK_SIZE;

use tokio::runtime::Runtime;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TcpClient {
    bridge: Option<Bridge>,
    runtime: Runtime,

    events: Vec<(String, Vec<Variant>)>,
    chunk_updates: Vec<ByteArray>,
}

#[methods]
impl TcpClient {
    fn new(_owner: &Node) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
            .unwrap();

        TcpClient { 
            bridge: None,
            runtime,

            events: Vec::new(),
            chunk_updates: Vec::new(),
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
    fn fetch_chunk_update(&mut self, _owner: &Node) -> Option<ByteArray> {
        self.chunk_updates.pop()
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
        if let Some(bridge) = &self.bridge {
            if let Some(event) = bridge.receive() {
                match event {
                    ServerToClient::ChunkUpdate(chunk) => {
                        let mut data: ByteArray = PoolArray::from_vec(vec![0; CHUNK_SIZE.pow(3)]);
                        for x in 0..CHUNK_SIZE {
                            for y in 0..CHUNK_SIZE {
                                for z in 0..CHUNK_SIZE {
                                    // https://coderwall.com/p/fzni3g/bidirectional-translation-between-1d-and-3d-arrays
                                    let i = x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE;
                                    data.set(i as i32, chunk.data[x][y][z].to_category().0);
                                }
                            }
                        }
                        self.chunk_updates.push(data);
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
}