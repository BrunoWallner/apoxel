// an improved channel with prioritisation

use crossbeam_channel::{Sender as CrossSender, Receiver as CrossReceiver, select, unbounded, bounded};

pub use crossbeam_channel::{SendError, RecvError, TryRecvError};

pub fn channel<T>(bound: Option<usize>) -> (Sender<T>, Receiver<T>) {
    let (
        (sender_0, receiver_0),
        (sender_1, receiver_1)
    ) = if let Some(bound) = bound {
        (
            bounded(bound),
            bounded(bound)
        )
    } else {
        (
            unbounded(),
            unbounded()
        )
    };
    (
        Sender { sender_0, sender_1 },
        Receiver{ receiver_0, receiver_1}
    )
}


#[derive(Debug, Clone)]
pub struct Sender<T> {
    sender_0: CrossSender<T>,
    sender_1: CrossSender<T>,
}
impl<T> Sender<T> {

    pub fn send(&self, t: T, important: bool) -> Result<(), SendError<T>> {
        if important {
            self.sender_0.send(t)
        } else {
            self.sender_1.send(t)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Receiver<T> {
    receiver_0: CrossReceiver<T>,
    receiver_1: CrossReceiver<T>,
}
impl<T> Receiver<T> {    

    pub fn recv(&self) -> Result<T, RecvError> {
        match self.receiver_0.try_recv() {
            Ok(t) => Ok(t),
            Err(e) => if !e.is_disconnected() {
                select! {
                    recv(self.receiver_0) -> t => t,
                    recv(self.receiver_1) -> t => t,
                }
            } else {
                Err(RecvError)
            }
        }
    }

    pub fn recv_with_meta(&self) -> Result<(T, bool), RecvError> {
        match self.receiver_0.try_recv() {
            Ok(t) => Ok((t, true)),
            Err(e) => if !e.is_disconnected() {
                select! {
                    recv(self.receiver_0) -> t => match t {
                        Ok(t) => Ok((t, true)),
                        Err(e) => Err(e),
                    },
                    recv(self.receiver_1) -> t => match t {
                        Ok(t) => Ok((t, false)),
                        Err(e) => Err(e),
                    },
                }
            } else {
                Err(RecvError)
            }
        }
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        match self.receiver_0.try_recv() {
            Ok(t) => Ok(t),
            Err(e) => if !e.is_disconnected() {
                self.receiver_1.try_recv()
            } else {
                Err(e)
            }
        }
    }

    pub fn try_recv_with_meta(&self) -> Result<(T, bool), TryRecvError> {
        match self.receiver_0.try_recv() {
            Ok(t) => Ok((t, true)),
            Err(e) => if !e.is_disconnected() {
                match self.receiver_1.try_recv() {
                    Ok(t) => Ok((t, false)),
                    Err(e) => Err(e)
                }
            } else {
                Err(e)
            }
        }
    }
}