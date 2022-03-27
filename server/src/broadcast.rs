use tokio::sync::mpsc;
use std::{marker::{Send, Sync}, fmt::Debug};

#[derive(Debug)]
pub enum Instruction<Event> {
    Send(Event),
    Register(mpsc::Sender<Event>),
}

// both global and client bound queue
#[derive(Clone, Debug)]
pub struct BroadCast<Event: Send> {
    sender: mpsc::Sender<Instruction<Event>>,
}
impl<Event: 'static + Send + Sync + Clone + Debug> BroadCast<Event> {
    pub fn init() -> Self {
        let (tx, rx) = mpsc::channel(4096);
        tokio::spawn(async move {
            init(rx).await;
        });
        Self {
            sender: tx,
        }
    }

    pub async fn send(&self, event: Event) {
        self.sender.send(Instruction::Send(event)).await.unwrap();
    }

    pub async fn register(&self, sender: mpsc::Sender<Event>) {
        self.sender.send(Instruction::Register(sender)).await.unwrap();
    }
}

async fn init<Event: 'static + Send + Sync + Clone + Debug>(mut receiver: mpsc::Receiver<Instruction<Event>>) {
    let mut senders: Vec<mpsc::Sender<Event>> = Vec::new();

    loop {
        match receiver.recv().await {
            Some(instruction) => match instruction {
                Instruction::Send(e) => {
                    let mut broken: Vec<usize> = Vec::new(); 
                    for (i, sender) in senders.iter().enumerate() {
                        let result = sender.send(e.clone()).await;
                        if result.is_err() {
                            broken.push(i);
                        }
                    }
                    // unregister broken sender
                    for broken in broken.iter() {
                        senders.remove(*broken);
                    }
                }
                Instruction::Register(sender) => {
                    senders.push(sender);
                }
            }
            None => {
                break;
            }
        }
    }
}