use tokio::sync::mpsc;
use super::Event;

use crate::chunks::handle::Handle as ChunkHandle;
use crate::player::handle::Handle as PlayerHandle;
use crate::broadcast::BroadCast;
use crate::events;

use crate::config::CONFIG;

use protocol::{
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
        chunk_handle.register_client(tx.clone()).await;
        init(rx, chunk_handle, player_handle, write_broadcast).await;

        super::init_pos_requester(tx.clone()).await;

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

async fn init(
    mut rx: mpsc::Receiver<Event>,
    chunk_handle: ChunkHandle,
    player_handle: PlayerHandle,
    write_broadcast: BroadCast<events::Tcp>,
) {
    tokio::spawn(async move {
        let mut player_pos: PlayerCoord = [0.0, 0.0, 0.0];
        let mut token: Option<Token> = None;

        loop {
            let received = rx.recv().await.unwrap();
            use Event::*;
            match received {
                RequestPlayerPos => {
                    if let Some(token) = token {
                        if let Some(player) = player_handle.get_player(token).await {
                            player_pos = player.pos;
                        }
                    }
                }
                ChunkUpdate(coord) => {
                    let pos = protocol::chunk::get_chunk_coords(&[player_pos[0] as i64, player_pos[1] as i64, player_pos[2] as i64]).0;
        
                    // check if coord is in range of player
                    let distance = (( 
                        (pos[0] - coord[0]).pow(2) +
                        (pos[1] - coord[1]).pow(2) +
                        (pos[2] - coord[2]).pow(2)
                    ) as f64).sqrt() as i64;
        
                    if distance <= CONFIG.chunks.render_distance as i64 {
                        let chunk = chunk_handle.request(vec![coord]).await[0];
                        write_broadcast.send(events::Tcp::Protocol(ProtocolEvent::ChunkUpdate(chunk))).await;
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