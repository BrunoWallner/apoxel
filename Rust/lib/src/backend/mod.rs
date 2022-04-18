// purposefully monolithic because of performance

pub mod client;
pub mod bridge;
mod chunk_mesh;

use bridge::Bridge;
use protocol::event::{ClientToServer, ServerToClient};
use protocol::chunk::Chunk;

use tokio::runtime::Runtime;
use gdnative::prelude::*;
use gdnative::profiler;

use std::thread;
use crossbeam::channel;

const MAX_CHUNKS_PER_CYCLE: usize = 50;
const MAX_EVENTS_PER_CYCLE: usize = 50;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Backend {
		bridge: Option<Bridge>,
		runtime: Runtime,

		events: Vec<(String, Vec<Variant>)>,
		//chunk_updates: Vec<(Coord, ByteArray)>,

		chunk_sender: channel::Sender<Chunk>,
		chunk_receiver: channel::Receiver<Option<Ref<Spatial>>>,
}

#[methods]
impl Backend {
		fn new(_owner: &Node) -> Self {
				let runtime = tokio::runtime::Builder::new_multi_thread()
						.enable_io()
						.build()
						.unwrap();

				// WARN: might lead to missing chunks if bounded
				let (chunk_sender, chunk_receiver) = channel::bounded(100);
				let (chunk_thread_sender, chunk_thread_receiver) = channel::bounded(100);

				init_chunk_thread(chunk_thread_receiver, chunk_sender);

				Self {
						bridge: None,
						runtime,

						events: Vec::new(),

						chunk_sender: chunk_thread_sender,
						chunk_receiver,
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
			profiler::profile(profiler::profile_sig!("Backend Events"), || {
				if let Some(bridge) = &self.bridge {
					for _ in 0..MAX_EVENTS_PER_CYCLE {

						if let Some(event) = bridge.receive() {
							match event {
								ServerToClient::ChunkUpdate(chunk) => {
									self.chunk_sender.send(chunk).unwrap();
								}
								ServerToClient::Token(token) => {
									self.events.push( (String::from("Token"), vec![Variant::new(token.to_vec())]) );
								}
								ServerToClient::Error(error) => {
									self.events.push( (String::from("Error"), vec![Variant::new(format!("{:?}", error))]) );
								}
							}
						} else {
							break;
						}
					}
				}
			});

			profiler::profile(profiler::profile_sig!("Backend Chunks"), || {
				self.spawn_chunks(owner);
			});
		}

		fn spawn_chunks(&self, owner: &Node) {
			for _ in 0..MAX_CHUNKS_PER_CYCLE {
				if let Ok(chunk) = self.chunk_receiver.try_recv() {
					if let Some(chunk) = chunk {
							owner.add_child(chunk, false);
					}
				} else {
					break;
				}
			}
		}
}

// TODO uitlize multiple threads
use threadpool::ThreadPool;
fn init_chunk_thread(
	chunk_receiver: channel::Receiver<Chunk>, 
	chunk_sender: channel::Sender<Option<Ref<Spatial>>>,
) {
	thread::spawn(move || {
		let threadpool = ThreadPool::new(8);
		loop {
			let chunk = chunk_receiver.recv().unwrap();
			let sender = chunk_sender.clone();
			threadpool.execute(move || {
				let chunk = chunk_mesh::gen::mesh(chunk);
				sender.send(chunk).unwrap();
			})
		}
	});
}