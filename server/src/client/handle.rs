use tokio::sync::mpsc;
use std::collections::HashSet;
use super::Event;

use crate::chunks::handle::Handle as ChunkHandle;
use crate::player::handle::Handle as PlayerHandle;
use crate::broadcast::BroadCast;
use crate::events;

use crate::config::CONFIG;

use protocol::{
    Coord,
    chunk::Chunk,
    Token,
    PlayerCoord, 
    event::Event as ProtocolEvent, 
    error::Error as ProtocolError
};

#[derive(Clone, Debug)]
pub struct Handle {
    pub sender: mpsc::Sender<Event>,
}
impl Handle {
    pub async fn init(
        chunk_handle: ChunkHandle,
        player_handle: PlayerHandle,
        write_broadcast: BroadCast<events::Tcp>,
    ) -> Self {
        let (tx, rx) = mpsc::channel(4096);
        init(rx, chunk_handle, player_handle, write_broadcast).await;

        super::init_chunk_requester(tx.clone()).await;

        Self {
            sender: tx,
        }
    }

    pub async fn register(&self, name: String) {
        self.sender.send(Event::Register{name}).await.unwrap();
    }
    pub async fn login(&self, token: Token) {
        self.sender.send(Event::Login(token)).await.unwrap();
    }
    pub async fn logoff(&self) {
        self.sender.send(Event::Logoff).await.unwrap();
    }
    pub async fn move_to(&self, pos: PlayerCoord) {
        self.sender.send(Event::Move(pos)).await.unwrap();
    }
}

async fn init_chunk_rx(
    mut rx: mpsc::Receiver<Chunk>,
    write_broadcast: BroadCast<events::Tcp>,
) {
    tokio::spawn(async move {
        loop {
            if let Some(chunk) = rx.recv().await {
                write_broadcast.send(events::Tcp::Protocol(ProtocolEvent::ChunkUpdate(chunk))).await;
            } else {
                // client disconnected
                break;
            }
        }
    });
}

async fn init(
    mut rx: mpsc::Receiver<Event>,
    chunk_handle: ChunkHandle,
    player_handle: PlayerHandle,
    write_broadcast: BroadCast<events::Tcp>,
) {
    tokio::spawn(async move {
        let mut player_pos: PlayerCoord = [0.0, 0.0, 0.0];
        let mut token: Option<Token> = None;

        let (chunk_tx, chunk_rx) = mpsc::channel(1024);
        init_chunk_rx(chunk_rx, write_broadcast.clone()).await;

        let mut loaded_chunks: HashSet<Coord> = HashSet::default();

        loop {
            let received = rx.recv().await.unwrap();
            use Event::*;
            match received {
                RequestChunks => {
                    if let Some(token) = token {
                        // get player pos
                        if let Some(player) = player_handle.get_player(token).await {
                            player_pos = player.pos;
                        }

                        // request chunks
                        let chunk_pos = protocol::chunk::get_chunk_coords(&[player_pos[0] as i64, player_pos[1] as i64, player_pos[2] as i64]).0;
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
                                    if !loaded_chunks.contains(&coord) {
                                        loaded_chunks.insert(coord);
                                        chunks.push(coord);
                                    }
                                }
                            }
                        }
                        if !chunks.is_empty() {
                            chunk_handle.request(chunks, token, chunk_tx.clone()).await;
                        }

                        // unload chunks
                        for coord in loaded_chunks.clone().into_iter() {
                            let distance = (( 
                                (coord[0] - chunk_pos[0]).pow(2) +
                                (coord[1] - chunk_pos[1]).pow(2) +
                                (coord[2] - chunk_pos[2]).pow(2)
                            ) as f64).sqrt() as i64;
                            if distance >= (CONFIG.chunks.render_distance as i64).pow(2) {
                                loaded_chunks.remove(&coord);
                            }
                        }
                    }
                }
                Register{name}=> {
                    if let Some(token) = player_handle.register(name).await {
                        write_broadcast.send(events::Tcp::Protocol(ProtocolEvent::Token(token))).await;
                    } else {
                        write_broadcast.send(events::Tcp::Protocol(ProtocolEvent::Error(ProtocolError::Register))).await;
                    }
                }
                Login(t) => {
                    if player_handle.login(t).await {
                        token = Some(t);
                    } else {
                        write_broadcast.send(events::Tcp::Protocol(ProtocolEvent::Error(ProtocolError::Login))).await;
                    }
                }
                Logoff => {
                    if let Some(token) = token {
                        player_handle.logoff(token).await;
                        break;
                    }
                }
                Move(pos) => {
                    if let Some(token) = token {
                        player_handle.move_player(token, pos).await;
                    }
                }
            }
        }
    });
}