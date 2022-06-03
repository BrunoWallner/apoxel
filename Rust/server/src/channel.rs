use crossbeam::channel;
use crossbeam::channel::SendError;

pub struct Receiver<T> {
    inner: channel::Receiver<T>,
}
impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        self.inner.recv().ok()
    }

    pub fn try_recv(&self) -> Option<T> {
        self.inner.try_recv().ok()
    }
}

pub struct Sender<T> {
    inner: channel::Sender<T>,
}
impl<T> Sender<T> {
    pub fn send(&self, data: T) -> Result<(), SendError<T>> {
        self.inner.send(data)
    }
}

pub fn new<T>() -> (Sender<T>, Receiver<T>) {
    let (s, r) = channel::unbounded();
    (
        Sender {inner: s},
        Receiver {inner: r},
    )
}