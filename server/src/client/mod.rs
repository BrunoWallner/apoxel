pub mod handle;

use protocol::{Coord, PlayerCoord, Token};
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum Event {
    RequestPlayerPos,
    ChunkUpdate(Coord),
    Register{name: String},
    Login(Token),
    Logoff,
    Move(PlayerCoord),
}

use std::time::Duration;
use tokio::{task, time};

async fn init_pos_requester(sender: mpsc::Sender<Event>) {
    tokio::spawn(async move {
        let forever = task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(1000));
    
            loop {
                interval.tick().await;
                let _ = sender.send(Event::RequestPlayerPos).await;
            }
        });
    
        forever.await.unwrap();
    });
}