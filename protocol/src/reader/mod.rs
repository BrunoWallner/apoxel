use tokio::io::{self, AsyncRead, AsyncReadExt};
use std::marker::Unpin;

use crate::header::Header;
use crate::error::Error;
use crate::Token;

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
        match self.socket.read(buffer).await {
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

    pub async fn get_header(&mut self) -> io::Result<Header> {
        let mut buf: [u8; 1] = [0; 1];
        self.read(&mut buf).await?;
        Ok(Header::from_value(buf[0]))
    }

    pub async fn get_string(&mut self) -> io::Result<String> {
        let mut buffer: Vec<u8> = Vec::new();
        'get_string: loop {
            let mut buf: [u8; 255] = [0; 255];
            self.socket.read(&mut buf).await?;
            let completed = buf[0] == 0x01;
            if completed {
                break 'get_string;
            }

            buffer.append(&mut buf[2..buf[1] as usize].to_vec());
        }

        Ok(String::from(String::from_utf8_lossy(&buffer)))
    }

    pub async fn get_token(&mut self) -> io::Result<Token> {
        let mut buf = [0_u8; 64];
        self.read(&mut buf).await?;

        Ok(buf)
    }

    pub async fn get_error(&mut self) -> io::Result<Error> {
        let mut buf = [0_u8; 1];
        self.read(&mut buf).await?;

        Ok(Error::from_value(buf[0]))
    }
}