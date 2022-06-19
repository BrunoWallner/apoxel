use crate::channel::*;
use std::fmt::Debug;
use crossbeam::channel::select;

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
            None => {
                select! {
                    recv(self.receiver_0.inner) -> t => t.ok(),
                    recv(self.receiver_1.inner) -> t => t.ok(),
                }
            }
        }
    }

    pub fn recv_with_meta(&self) -> Option<(T, bool)> {
        match self.receiver_0.try_recv() {
            Some(t) => Some((t, true)),
            None => {
                select! {
                    recv(self.receiver_0.inner) -> t => match t {
                        Ok(t) => Some((t, true)),
                        Err(_) => None,
                    },
                    recv(self.receiver_1.inner) -> t => match t {
                        Ok(t) => Some((t, false)),
                        Err(_) => None,
                    },
                }
            }
        }
    }

    pub fn try_recv(&self) -> Option<T> {
        match self.receiver_0.try_recv() {
            Some(t) => Some(t),
            None => self.receiver_1.try_recv()
        }
    }

    pub fn try_recv_with_meta(&self) -> Option<(T, bool)> {
        match self.receiver_0.try_recv() {
            Some(t) => Some((t, true)),
            None => match self.receiver_1.try_recv() {
                Some(t) => Some((t, false)),
                None => None
            }
        }
    }
}