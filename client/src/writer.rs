use protocol::writer::Writer as ProtocolWriter;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc;

use protocol::{Token, PlayerCoord};
use protocol::event::Event;

#[derive(Clone, Debug)]
pub enum Instruction {
    Register(String),
    Login(Token),
    Move(PlayerCoord),
}

#[derive(Clone, Debug)]
pub struct Writer {
    pub sender: mpsc::Sender<Instruction>
}
impl Writer {
    pub fn init(writer: OwnedWriteHalf) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        init(ProtocolWriter::new(writer), rx);
        Self {
            sender: tx,
        }
    }
}

fn init(mut writer: ProtocolWriter<OwnedWriteHalf>, mut rx: mpsc::Receiver<Instruction>) {
    #[allow(unreachable_code)]
    tokio::spawn(async move {
        loop {
            match rx.recv().await.unwrap() {
                Instruction::Register(name) => {
                    writer.send_event(&Event::Register{name}).await?;
                }
                Instruction::Login(token) => {
                    writer.send_event(&Event::Login{token}).await?;
                }
                Instruction::Move(pos) => {
                    writer.send_event(&Event::MovePlayer(pos)).await?;
                }
            }
        }
        Ok::<_, tokio::io::Error>(())
    });
}