use crossbeam::channel;
use super::GameEvent;

#[derive(Clone, Debug)]
pub enum Instruction {
    Push(GameEvent),
    Pull{amount: usize, sender: channel::Sender<Vec<GameEvent>>},
}

#[derive(Clone, Debug)]
pub struct Queue {
    sender: channel::Sender<Instruction>,
}
impl Queue {
    pub fn init() -> Self {
        let (tx, rx) = channel::unbounded();
        init(rx);
        Self{sender: tx}
    }
    pub fn send(&self, event: GameEvent) {
        self.sender.send(Instruction::Push(event)).unwrap();
    }
    pub fn pull(&self, amount: usize) -> Vec<GameEvent> {
        let (tx, rx) = channel::unbounded();
        self.sender.send(Instruction::Pull{amount, sender: tx}).unwrap();
        rx.recv().unwrap()
    }
}

fn init(rx: channel::Receiver<Instruction>) {
    tokio::spawn(async move {
        let mut queue: Vec<GameEvent> = Vec::new();

        loop {
            match rx.recv().unwrap() {
                Instruction::Push(ev) => {
                    queue.push(ev);
                }
                Instruction::Pull{amount, sender} => {
                    if queue.len() > amount {
                        sender.send(queue.drain(0..amount).as_slice().to_vec()).unwrap();
                    } else {
                        sender.send(queue.drain(..).as_slice().to_vec()).unwrap();
                    }
                }
            }
        }
    });
}