use tokio::io::{self, AsyncRead, AsyncReadExt};
use std::marker::Unpin;
use crate::event::Event;
use crate::TCP_EVENT_BYTES;
use tokio::io::BufReader;

pub struct Reader<T: AsyncRead + Unpin> {
    socket: BufReader<T>,
    bytes_read: u128, // overkill
}
impl<T: AsyncRead + Unpin> Reader<T> {
    pub fn new(socket: T) -> Self {
        Self {
            socket: BufReader::new(socket),
            bytes_read: 0,
        }
    }

    // when len is 0 stream is closed
    // have to manually return error to remove 100% cpu cycle
    pub async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self.socket.read_exact(buffer).await {
            Ok(0) => {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe"));
            }
            Ok(len) => {
                self.bytes_read += TCP_EVENT_BYTES as u128;
                return Ok(len);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn get_event(&mut self) -> io::Result<Event> {
        let mut buffer: Vec<u8> = Vec::new();
        'get_bytes: loop {
            let mut buf = Box::new([0u8; TCP_EVENT_BYTES]);
            self.read(&mut (*buf)).await?;
            let completed = buf[0] == 1;

            buffer.append(&mut buf[2..buf[1] as usize].to_vec());

            if completed {
                break 'get_bytes;
            }
        }

        let event: Event = bincode::deserialize(&buffer[..]).unwrap_or(Event::Invalid);
        Ok(event)
    }

    pub fn bytes_read(&self) -> u128 {
        self.bytes_read
    }
}