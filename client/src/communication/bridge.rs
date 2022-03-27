use protocol::event::Event as TcpEvent;
use tokio::sync::mpsc;
use super::GameEvent;

#[derive(Clone, Debug)]
pub struct Bridge {
    tcp_sender: mpsc::Sender<TcpEvent>,
    game_sender: mpsc::Sender<GameEvent>,
}
impl Bridge {
    pub fn init() -> (Self, mpsc::Receiver<TcpEvent>, mpsc::Receiver<GameEvent>) {
        let (tcp_tx, tcp_rx) = mpsc::channel(1024);
        let (game_tx, game_rx) = mpsc::channel(1024);
        (
            Self {tcp_sender: tcp_tx, game_sender: game_tx},
            tcp_rx,
            game_rx,
        )
    }

    pub async fn push_tcp(&self, event: TcpEvent) {
        self.tcp_sender.send(event).await.unwrap();
    }
    pub async fn push_game(&self, event: GameEvent) {
        self.game_sender.send(event).await.unwrap();
    }
}