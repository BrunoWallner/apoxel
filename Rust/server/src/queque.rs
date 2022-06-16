use crate::channel::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Queque<T> {
    // priority 0
    sender_0: Sender<T>,
    receiver_0: Receiver<T>,
    // priority 1
    sender_1: Sender<T>,
    receiver_1: Receiver<T>,
}
impl<T: Debug> Queque<T> {
    pub fn new() -> Self {
        let (sender_0, receiver_0) = channel();
        let (sender_1, receiver_1) = channel();
        Self {
            sender_0,
            receiver_0,
            sender_1,
            receiver_1,
        }
    }
    pub fn send(&self, t: T, important: bool) -> Result<(), ()> {
        if important {
            self.sender_0.send(t)
        } else {
            self.sender_1.send(t)
        }
    }

    pub fn recv(&self) -> Option<T> {
        match self.receiver_0.try_recv() {
            Some(t) => Some(t),
            None => self.receiver_1.recv()
        }
    }

    pub fn try_recv(&self) -> Option<T> {
        match self.receiver_0.try_recv() {
            Some(t) => Some(t),
            None => self.receiver_1.try_recv()
        }
    }
}