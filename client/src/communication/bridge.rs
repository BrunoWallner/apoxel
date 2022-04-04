use protocol::event::Event as TcpEvent;
use super::GameEvent;

use crossbeam::channel;

#[derive(Clone, Debug)]
pub struct Bridge {
    tcp_sender: channel::Sender<TcpEvent>,
    game_sender: channel::Sender<GameEvent>,
}
impl Bridge {
    pub fn init() -> (Self, channel::Receiver<TcpEvent>, channel::Receiver<GameEvent>) {
        let (tcp_tx, tcp_rx) = channel::unbounded();
        let (game_tx, game_rx) = channel::unbounded();
        (
            Self {tcp_sender: tcp_tx, game_sender: game_tx},
            tcp_rx,
            game_rx,
        )
    }

    pub fn push_tcp(&self, event: TcpEvent) {
        let _ = self.tcp_sender.send(event);
    }
    pub fn push_game(&self, event: GameEvent) {
        self.game_sender.send(event).unwrap();
    }
}