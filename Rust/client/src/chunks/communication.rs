use protocol::prelude::*;
use bevy::prelude::Mesh;
use std::collections::BTreeMap;
use std::thread;

#[derive(Clone, Debug)]
enum InternalEvent {
    Unload(Vec<Coord>),
    Load(Vec<Chunk>),
    Update(Vec<ChunkDelta>),
}

#[derive(Clone, Debug)]
pub enum ExternalEvent {
    Load((Coord, Mesh)),
    Unload(Vec<Coord>),
}

#[derive(Clone, Debug)]
pub struct ChunkCommunicator {
    sender: Sender<InternalEvent>,
    receiver: Receiver<ExternalEvent>,
}
impl ChunkCommunicator {
    pub fn new() -> Self {
        let (internal_tx, internal_rx) = channel(Some(64));
        let (external_tx, external_rx) = channel(Some(64));
        init(external_tx, internal_rx);
        Self {
            sender: internal_tx,
            receiver: external_rx,
        }
    }

    pub fn load(&self, chunks: Vec<Chunk>) {
        self.sender.send(InternalEvent::Load(chunks), false).unwrap();
    }

    pub fn update(&self, deltas: Vec<ChunkDelta>) {
        self.sender.send(InternalEvent::Update(deltas), false).unwrap();
    }

    pub fn unload(&self, coords: Vec<Coord>) {
        self.sender.send(InternalEvent::Unload(coords), false).unwrap();
    }

    #[allow(dead_code)]
    pub fn get(&self) -> Option<ExternalEvent> {
        self.receiver.recv().ok()
    }

    pub fn try_get(&self) -> Option<ExternalEvent> {
        match self.receiver.try_recv() {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }
}

fn init(
    sender: Sender<ExternalEvent>,
    receiver: Receiver<InternalEvent>
) {
    thread::spawn(move || {
        let pool = threadpool::ThreadPool::new(8);
        let mut chunks: BTreeMap<Coord, Chunk> = BTreeMap::default();
        loop {
            match receiver.recv().unwrap() {
                InternalEvent::Load(mut chunk_loads) => {
                    for chunk in chunk_loads.iter_mut() {
                        //  if chunkupdate was faster than chunkload
                        if let Some(c) = chunks.get(&chunk.coord) {
                            chunk.merge(c);
                            let chunk_clone = chunk.clone();
                            let sender_clone = sender.clone();
                            pool.execute(move || {
                                let mesh = super::mesh::generate(&chunk_clone);
                                sender_clone.send(ExternalEvent::Load((chunk_clone.coord, mesh)), false).unwrap();
                            });
                            chunks.insert(chunk.coord, chunk.clone());
                        } else {
                            let chunk_clone = chunk.clone();
                            let sender_clone = sender.clone();
                            pool.execute(move || {
                                let mesh = super::mesh::generate(&chunk_clone);
                                sender_clone.send(ExternalEvent::Load((chunk_clone.coord, mesh)), false).unwrap();
                            });
                            chunks.insert(chunk.coord, chunk.clone());
                        }
                    }
                },
                InternalEvent::Update(deltas) => {
                    for delta in deltas.iter() {
                        //  if chunkupdate was faster than chunkload
                        if let Some(c) = chunks.get_mut(&delta.0) {
                            c.apply_delta(delta);
                            let chunk_clone = c.clone();
                            let sender_clone = sender.clone();
                            pool.execute(move || {
                                let mesh = super::mesh::generate(&chunk_clone);
                                sender_clone.send(ExternalEvent::Load((chunk_clone.coord, mesh)), true).unwrap();
                            });
                        } else {
                            let mut chunk = Chunk::new(delta.0);
                            chunk.apply_delta(delta);
                            chunks.insert(delta.0, chunk.clone());
                            let sender_clone = sender.clone();
                            let coord = delta.0;
                            pool.execute(move || {
                                let mesh = super::mesh::generate(&chunk);
                                sender_clone.send(ExternalEvent::Load((coord, mesh)), true).unwrap();
                            });
                        }
                    }
                }
                InternalEvent::Unload(coords) => {
                    sender.send(ExternalEvent::Unload(coords), false).unwrap();
                }
            }
        }
    });
}