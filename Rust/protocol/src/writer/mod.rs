use tokio::io::{self, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;
use crate::TCP_EVENT_BYTES;
use crate::event::Event;
use tokio::io::BufWriter;

static mut WRITTEN: u64 = 0;

pub struct Writer<T: AsyncWrite + Unpin> {
    socket: BufWriter<T>,
    bytes_written: u128, // overkill
}
impl<T: AsyncWrite + Unpin> Writer<T> {
    pub fn new(socket: T) -> Self {
        Self {
            socket: BufWriter::new(socket),
            bytes_written: 0,
        }
    }

    pub async fn write(&mut self, buffer: &[u8]) -> io::Result<()> {
        unsafe {WRITTEN += 1}
        // unsafe {log::info!("written: {} MB", WRITTEN as f64 / 1000000.0)}
        self.socket.write_all(buffer).await?;
        Ok(())
    }

    pub async fn send_event(&mut self, event: &Event) -> io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(event).unwrap();
        let bytes = byte_vector(&encoded);
        for bytes in bytes {
            self.write(&bytes).await?;
            self.bytes_written += TCP_EVENT_BYTES as u128;
        }
        self.socket.flush().await?;

        Ok(())
    }

    pub fn bytes_written(&self) -> u128 {
        self.bytes_written
    }
}

fn byte_vector(bytes: &[u8]) -> Vec<Vec<u8>> {
    let buffer_size = TCP_EVENT_BYTES - 2; // because of first byte that carries extra info

    let mut buffer: Vec<Vec<u8>> = Vec::new();

    let mut chunks = bytes.chunks(buffer_size).peekable();

    loop {
        if let Some(chunk) = chunks.next() {
            let len = chunk.len() as u8;
            let mut buf: Vec<u8> = Vec::new();

            let last = !chunks.peek().is_some();
            if last {
                buf.push(1);
            } else {
                buf.push(0);
            }

            buf.push(len + 2);
        
            buf.append(&mut chunk.to_vec());
            buf.append(&mut vec![0x00; buffer_size - chunk.len()]);
            buffer.push(buf);

            if last {break}
        }
    }

    buffer
}