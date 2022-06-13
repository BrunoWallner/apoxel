use crossbeam::channel;
use crossbeam::channel::SendError;

#[derive(Debug, Clone)]
pub struct Receiver<T> {
    inner: channel::Receiver<T>,
}
impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        self.inner.recv().ok()
    }

    #[allow(dead_code)]
    pub fn try_recv(&self) -> Option<T> {
        self.inner.try_recv().ok()
    }
}

#[derive(Debug, Clone)]
pub struct Sender<T> {
    inner: channel::Sender<T>,
}
impl<T> Sender<T> {
    pub fn send(&self, data: T) -> Result<(), SendError<T>> {
        self.inner.send(data)
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (s, r) = channel::unbounded();
    (
        Sender {inner: s},
        Receiver {inner: r},
    )
}

// will block if full
pub fn bounded_channel<T>(bound: usize) -> (Sender<T>, Receiver<T>) {
    let (s, r) = channel::bounded(bound);
    (
        Sender {inner: s},
        Receiver {inner: r},
    )
}