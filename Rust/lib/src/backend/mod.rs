// purposefully monolithic because of performance

pub mod bridge;
mod chunk_handle;
pub mod client;

use bridge::Bridge;
use protocol::chunk::Chunk;
use protocol::event::{ClientToServer, ServerToClient};

use gdnative::prelude::*;
use tokio::runtime::Runtime;

use crossbeam::channel;
use std::thread;
use std::time::Instant;

use crate::terminator::Terminator;

use gdnative::profiler;

const MAX_CHUNK_TIME: u128 = 8000; // in µs

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Backend {
    bridge: Option<Bridge>,
    runtime: Runtime,

    chunk_handle: Option<chunk_handle::ChunkHandle>,
    event_receiver: Option<channel::Receiver<(String, Vec<Variant>)>>,

    terminator: Terminator,
}

#[methods]
impl Backend {
    fn new(_owner: &Node) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
			.enable_time()
            .build()
            .unwrap();

        Self {
            bridge: None,
            runtime,

            chunk_handle: None,
            event_receiver: None,

            terminator: Terminator::new(),
        }
    }

    #[export]
    fn terminate(&mut self, _owner: &Node) {
        self.terminator.terminate();
    }

    #[export]
    fn establish_connection(&mut self, _owner: &Node, host: String) -> bool {
        if let Some(bridge) = client::init(host, &self.runtime, self.terminator.clone()) {
            self.bridge = Some(bridge.clone());

            let (event_sender, event_receiver) = channel::bounded(100);

            let chunk_handle = chunk_handle::ChunkHandle::init(self.terminator.clone());
            let chunk_sender = chunk_handle.chunk_sender.clone();
            self.chunk_handle = Some(chunk_handle);

            init_event_handle(
                bridge,
                event_sender,
                chunk_sender,
                self.terminator.clone(),
            );

            self.event_receiver = Some(event_receiver);

            true
        } else {
            false
        }
    }

    #[export]
    fn connection_established(&self, _owner: &Node) -> bool {
        self.bridge.is_some()
    }

    #[export]
    fn fetch_event(&mut self, _owner: &Node) -> Option<(String, Vec<Variant>)> {
        if let Some(event_receiver) = &self.event_receiver {
            match event_receiver.try_recv() {
                Ok(ev) => Some(ev),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    #[export]
    fn send(&self, _owner: &Node, event: String) -> bool {
        if let Some(bridge) = &self.bridge {
            let ev: Result<ClientToServer, serde_json::Error> = serde_json::from_str(&event);

            match ev {
                Ok(event) => {
                    bridge.send(event);
                }
                Err(e) => {
                    godot_print!("attempted to send invalid tcp event:\n{}\n{:?}", e, event);
                }
            }
            true
        } else {
            false
        }
    }

    #[export]
    fn _process(&mut self, owner: &Node, _dt: f64) {
		profiler::profile(profiler::profile_sig!("Chunk Spawing"), || {
			self.spawn_chunks(owner);
		});
    }

    fn spawn_chunks(&self, owner: &Node) {
        if let Some(chunk_handle) = &self.chunk_handle {
            let start = Instant::now();
            'spawning: loop {
                if let Ok(chunk) = chunk_handle.chunk_mesh_receiver.try_recv() {
                    if let Some(chunk) = chunk {
                        owner.add_child(chunk, false);
                    }
                } else {
                    break 'spawning;
                }

                let fin = start.elapsed().as_micros();
                if fin >= MAX_CHUNK_TIME {
                    break 'spawning;
                }
            }
        }
    }
}

fn init_event_handle(
    bridge: Bridge,
    event_sender: channel::Sender<(String, Vec<Variant>)>,
    chunk_sender: channel::Sender<Chunk>,
    terminator: Terminator,
) {
    thread::spawn(move || loop {
        if terminator.should_terminate() {
            break;
        }
        if let Some(event) = bridge.receive() {
            match event {
                ServerToClient::ChunkUpdate(chunk) => {
                    let _ = chunk_sender.send(chunk);
                }
                ServerToClient::Token(token) => {
                    let _ = event_sender
                        .send((String::from("Token"), vec![Variant::new(token.to_vec())]));
                }
                ServerToClient::Error(error) => {
                    let _ = event_sender.send((
                        String::from("Error"),
                        vec![Variant::new(format!("{:?}", error))],
                    ));
                }
            }
        }
    });
}
