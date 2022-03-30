use tokio::sync::mpsc;
use protocol::{Token, Coord, chunk::Chunk};
use crate::config::CONFIG;

use crate::chunks::handle::Handle as ChunkHandle;
use crate::broadcast::BroadCast;
use protocol::event::Event as ProtocolEvent;
use crate::events;

use std::collections::HashSet;

use super::Event;


#[derive(Copy, Clone, Debug)]
pub enum ChunkEvent {
    Request(Coord),
    PushLoadedChunk(Chunk),
    RequestUnload(Coord),
    SetToken(Token),
}

pub struct Handle {
    pub sender: mpsc::Sender<ChunkEvent>
}
impl Handle {
    pub async fn init(chunk_handle: ChunkHandle, write_broadcast: BroadCast<events::Tcp>, event_sender: mpsc::Sender<Event>) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        //init_chunk_rx(chunk_rx, write_broadcast.clone(), tx.clone()).await;
        init(tx.clone(), rx, chunk_handle, write_broadcast.clone());
        init_chunk_requester(event_sender).await;
        Self {sender: tx}
    }
}

use std::time::Duration;
use tokio::{task, time};

async fn init_chunk_requester(sender: mpsc::Sender<Event>) {
    tokio::spawn(async move {
        let forever = task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(25));
    
            loop {
                interval.tick().await;
                let _ = sender.send(Event::RequestChunks).await;
            }
        });
    
        forever.await.unwrap();
    });
}

fn init(tx: mpsc::Sender<ChunkEvent>, mut rx: mpsc::Receiver<ChunkEvent>, chunk_handle: ChunkHandle, write_broadcast: BroadCast<events::Tcp>) {
    tokio::spawn(async move {
        let mut loaded: HashSet<Coord> = HashSet::default();
        let mut token: Option<Token> = None;

        loop {
            match rx.recv().await.unwrap() {
                ChunkEvent::SetToken(t) => {
                    token = Some(t);
                    chunk_handle.register_update_notifier(t, tx.clone()).await;
                }
                ChunkEvent::Request(chunk_pos) => {
                    if let Some(token) = token {
                        let rd = CONFIG.chunks.render_distance as i64;
                        let mut chunks: Vec<Coord> = Vec::new();
                        for x in -rd..rd {
                            for y in -rd..rd {
                                for z in -rd..rd {
                                    let coord = [x + chunk_pos[0], y + chunk_pos[1], z + chunk_pos[2]];
                                    let distance = (( 
                                        (coord[0] - chunk_pos[0]).pow(2) +
                                        (coord[1] - chunk_pos[1]).pow(2) +
                                        (coord[2] - chunk_pos[2]).pow(2)
                                    ) as f64).sqrt() as i64;
                                    if distance >= rd {
                                        continue;
                                    }
                                    if !loaded.contains(&coord) {
                                        chunks.push(coord);
                                    }
                                }
                            }
                        }
                        if !chunks.is_empty() {
                            chunks.sort_unstable_by_key(|coord| {
                                (coord[0] - chunk_pos[0]).pow(2) +
                                (coord[1] - chunk_pos[1]).pow(2) +
                                (coord[2] - chunk_pos[2]).pow(2)
                            });
                            chunks.reverse();
                            chunk_handle.request(chunks, token).await;
                        }
                    }                   
                }
                ChunkEvent::RequestUnload(chunk_pos) => {
                    // unload chunks
                    for coord in loaded.clone().into_iter() {
                        let distance = (( 
                            (coord[0] - chunk_pos[0]).pow(2) +
                            (coord[1] - chunk_pos[1]).pow(2) +
                            (coord[2] - chunk_pos[2]).pow(2)
                        ) as f64).sqrt() as i64;
                        if distance >= CONFIG.chunks.render_distance as i64 {
                            loaded.remove(&coord);
                        }
                    }
                }
                ChunkEvent::PushLoadedChunk(chunk) => {
                    loaded.insert(chunk.coord);
                    write_broadcast.send(events::Tcp::Protocol(ProtocolEvent::ChunkUpdate(chunk))).await;
                }
            }
        }
    });
}