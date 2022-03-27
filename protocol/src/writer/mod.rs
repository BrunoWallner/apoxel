use tokio::io::{self, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

use crate::event::Event;

pub struct Writer<T: AsyncWrite + Unpin> {
    socket: T,
}
impl<T: AsyncWrite + Unpin> Writer<T> {
    pub fn new(socket: T) -> Self {
        Self {
            socket,
        }
    }

    pub async fn write(&mut self, buffer: &[u8]) -> io::Result<()> {
        self.socket.write_all(buffer).await?;
        Ok(())
    }

    pub async fn send_event(&mut self, event: &Event) -> io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(event).unwrap();
        let bytes = byte_vector(&encoded);
        for bytes in bytes {
            self.write(&bytes).await?;
        }

        Ok(())
    }
}

fn byte_vector(bytes: &[u8]) -> Vec<Vec<u8>> {
    let buffer_size = 255 - 2; // because of first 2 bytes that carry extra info

    let mut buffer: Vec<Vec<u8>> = Vec::new();

    for chunk in bytes.chunks(buffer_size) {
        let len = (chunk.len() + 2) as u8;
        let mut buf: Vec<u8> = Vec::new();
        buf.append(&mut [0x00, len].to_vec()); // indicates that no end and len of data

        buf.append(&mut chunk.to_vec());
        buf.append(&mut vec![0x00; 253 - chunk.len()]);
        buffer.push(buf);
    }
    buffer.push(vec![0x01; 255]);

    buffer
}