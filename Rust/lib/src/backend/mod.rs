// purposefully monolithic because of performance

pub mod client;
pub mod bridge;
mod chunk_mesh;

use bridge::Bridge;
use protocol::event::{ClientToServer, ServerToClient};
use protocol::chunk::Chunk;

use tokio::runtime::Runtime;
use gdnative::prelude::*;

use std::thread;
use std::time::Instant;
use crossbeam::channel;

use crate::terminator::Terminator;

const MAX_CHUNK_TIME: u128 = 1000; // in µs

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Backend {
		bridge: Option<Bridge>,
		runtime: Runtime,

		event_receiver: Option<channel::Receiver<(String, Vec<Variant>)>>,
		chunk_mesh_receiver: Option<channel::Receiver<Option<Ref<Spatial>>>>,

		terminator: Terminator,
}

#[methods]
impl Backend {
		fn new(_owner: &Node) -> Self {
				let runtime = tokio::runtime::Builder::new_multi_thread()
						.enable_io()
						.build()
						.unwrap();

				Self {
						bridge: None,
						runtime,

						event_receiver: None,
						chunk_mesh_receiver: None,

						terminator: Terminator::new(),
				}
		}

		#[export]
		fn terminate(&mut self, _owner: &Node) {
			self.terminator.terminate();
		}

		#[export]
		fn establish_connection(&mut self, _owner: &Node, host: String) -> bool {
				if let Some( bridge ) = client::init(host, &self.runtime, self.terminator.clone()) {
						self.bridge = Some(bridge.clone());

						// WARN: might lead to missing chunks if bounded
						let (chunk_sender, chunk_receiver) = channel::bounded(100);
						let (chunk_thread_sender, chunk_thread_receiver) = channel::bounded(100);
						let (event_sender, event_receiver) = channel::bounded(100);

						self.event_receiver = Some(event_receiver);
						self.chunk_mesh_receiver = Some(chunk_receiver);

						chunk_mesh::init_generation(chunk_thread_receiver, chunk_sender, self.terminator.clone());
						init_event_handle(
							bridge.clone(),
							event_sender,
							chunk_thread_sender,
							self.terminator.clone(),
						);

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
			if let Some(event_receiver) = &self.event_receiver {
				match event_receiver.try_recv() {
					Ok(ev) => Some(ev),
					Err(_) => None
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
		fn _process(&mut self, owner: &Node, _dt: f64) {
			self.spawn_chunks(owner);
		}

		fn spawn_chunks(&self, owner: &Node) {
			if let Some(chunk_mesh_receiver) = &self.chunk_mesh_receiver {
				let start = Instant::now();
				'spawning: loop {
					if let Ok(chunk) = chunk_mesh_receiver.try_recv() {
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
					let _ = event_sender.send( (String::from("Token"), vec![Variant::new(token.to_vec())]) );
				}
				ServerToClient::Error(error) => {
					let _ = event_sender.send( (String::from("Error"), vec![Variant::new(format!("{:?}", error))]) );
				}
			}
		}
	});
}