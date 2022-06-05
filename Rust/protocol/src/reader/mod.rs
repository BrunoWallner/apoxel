use tokio::io::{self, AsyncRead, AsyncReadExt};
use std::marker::Unpin;

use crate::event::Event;
pub struct Reader<T: AsyncRead + Unpin> {
    socket: T,
}
impl<T: AsyncRead + Unpin> Reader<T> {
    pub fn new(socket: T) -> Self {
        Self {
            socket,
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
            let mut buf = [0u8; 255];
            self.read(&mut buf).await?;
            let completed = buf[0] == 1;

            buffer.append(&mut buf[2..buf[1] as usize].to_vec());

            if completed {
                break 'get_bytes;
            }
            break
        }

        let event: Event = bincode::deserialize(&buffer[..]).unwrap_or(Event::Invalid);
        Ok(event)
    }
}