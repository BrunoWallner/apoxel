use tokio::io::{self, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

use crate::header::Header;
use crate::error::Error;
use crate::Token;

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
        self.socket.write_all(buffer).await
    }

    pub async fn send_header(&mut self, header: &Header) -> io::Result<()> {
        self.write(&[header.to_value()]).await?;

        Ok(())
    }

    pub async fn send_string(&mut self, string: String) -> io::Result<()> {
        fn string_to_bytes(string: String) -> Vec<Vec<u8>> {
            let buffer_size = 255 - 2; // because of first 2 bytes that carry extra info
            let bytes = string.as_bytes().to_vec();
        
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
        let bytes = string_to_bytes(string);
        for bytes in bytes {
            self.write(&bytes).await?;
        }

        Ok(())
    }

    pub async fn send_token(&mut self, token: &Token) -> io::Result<()> {
        self.send_header(&Header::ReceiveToken).await?;
        self.socket.write_all(token).await?;

        Ok(())
    }

    pub async fn send_error(&mut self, error: &Error) -> io::Result<()> {
        self.send_header(&Header::Error).await?;
        self.write(&[error.to_value()]).await?;

        Ok(())
    }

}