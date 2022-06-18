use crate::channel::*;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Queque<T> {
    // priority 0
    sender_0: Sender<T>,
    receiver_0: Receiver<T>,
    // priority 1
    sender_1: Sender<T>,
    receiver_1: Receiver<T>,
    // channel len
    len: Arc<Mutex<usize>>,
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
            len: Arc::new(Mutex::new(0)),
        }
    }

    // pub fn send(&self, t: T, _important: bool) -> Result<(), ()> {
    //     self.sender_0.send(t);
    //     Ok(())
    // }

    // pub fn recv(&self) -> Option<T> {
    //     self.receiver_0.recv()
    // }

    // pub fn try_recv(&self) -> Option<T> {
    //     self.receiver_0.try_recv()
    // }

    fn increase_len(&self) {
        *self.len.lock().unwrap() += 1;
    }

    fn decrease_len(&self) {
        *self.len.lock().unwrap() -= 1;
    }

    pub fn len(&self) -> usize {
        *self.len.lock().unwrap()
    }
    
    pub fn send(&self, t: T, important: bool) -> Result<(), ()> {
        // when all channels are empty the recv() func would block on
        // non important t's, so sending the important t to the unimportant
        // channel is better -> looses potential priority info
        let t = if self.len() == 0 {
            self.sender_1.send(t)
        } else {
            if important {
                self.sender_0.send(t)
            } else {
                self.sender_1.send(t)
            }
        };
        self.increase_len();
        t
    }

    pub fn recv(&self) -> Option<T> {
        let t = match self.receiver_0.try_recv() {
            Some(t) => Some(t),
            None => self.receiver_1.recv()
        };
        self.decrease_len();
        t
    }

    pub fn try_recv(&self) -> Option<T> {
        match self.receiver_0.try_recv() {
            Some(t) => {
                self.decrease_len();
                Some(t)
            },
            None => match self.receiver_1.try_recv() {
                Some(t) => {
                    self.decrease_len();
                    Some(t)
                },
                None => None
            }
        }
    }
}