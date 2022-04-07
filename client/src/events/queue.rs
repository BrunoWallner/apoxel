// Allows non blocking communcation to game loop

/*

use crossbeam::channel;
use super::GameEvent;

use std::thread;

#[derive(Clone, Debug)]
enum Instruction {
    Push(GameEvent),
    Pull{amount: usize, sender: channel::Sender<Vec<GameEvent>>},
}

#[derive(Clone, Debug)]
pub struct GameEventQueue {
    sender: channel::Sender<Instruction>,
}
impl GameEventQueue {
    pub fn init() -> Self {
        let (tx, rx) = channel::unbounded();
        init(rx);
        Self{sender: tx}
    }
    pub fn push(&self, event: GameEvent) {
        self.sender.send(Instruction::Push(event)).unwrap();
    }
    pub fn pull(&self, amount: usize) -> Vec<GameEvent> {
        let (tx, rx) = channel::unbounded();
        self.sender.send(Instruction::Pull{amount, sender: tx}).unwrap();
        rx.recv().unwrap()
    }
}

fn init(rx: channel::Receiver<Instruction>) {
    thread::spawn(move || {
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
*/


use std::sync::{Mutex, Arc};
use super::GameEvent;

#[derive(Clone, Debug)]
pub struct GameEventQueue {
    queue: Arc<Mutex<Vec<GameEvent>>>
}
impl GameEventQueue {
    pub fn init() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn push(&mut self, event: GameEvent) {
        let mut queue = self.queue.lock().unwrap();
        queue.insert(0, event);
    }

    pub fn pull(&mut self) -> Option<GameEvent> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }
}