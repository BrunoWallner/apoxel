use tokio::sync::mpsc;
use super::GameEvent;

#[derive(Clone, Debug)]
pub enum Instruction {
    Push(GameEvent),
    Pull(mpsc::Sender<Option<GameEvent>>),
}

#[derive(Clone, Debug)]
pub struct Queue {
    sender: mpsc::Sender<Instruction>,
}
impl Queue {
    pub fn init() -> Self {
        let (tx, rx) = mpsc::channel(1024);
        init(rx);
        Self{sender: tx}
    }
    pub async fn send(&self, event: GameEvent) {
        self.sender.send(Instruction::Push(event)).await.unwrap();
    }
    pub async fn pull(&self) -> Option<GameEvent> {
        println!("new pull");
        let (tx, mut rx) = mpsc::channel(1);
        self.sender.send(Instruction::Pull(tx)).await.unwrap();
        rx.recv().await.unwrap()
    }
}

fn init(mut rx: mpsc::Receiver<Instruction>) {
    tokio::spawn(async move {
        let mut queue: Vec<GameEvent> = Vec::new();

        loop {
            match rx.recv().await.unwrap() {
                Instruction::Push(ev) => {
                    queue.push(ev);
                }
                Instruction::Pull(sender) => {
                    sender.send(queue.pop()).await.unwrap();
                }
            }
        }
    });
}