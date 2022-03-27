mod writer;
use writer::Instruction;

use tokio::net::TcpStream;

use std::error::Error;

use protocol::{Token, reader, event::Event};

use std::time::Duration;
use std::thread::sleep;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("0.0.0.0:8000").await?;
    let (read, write) = stream.into_split();
    let mut reader = reader::Reader::new(read);
    let mut writer = writer::Writer::init(write);

    writer.sender.send(Instruction::Register(String::from("luc23a2d"))).await?;


    let writer_clone = writer.clone();
    #[allow(unreachable_code)]
    tokio::spawn(async move {
        let mut token: Option<Token> = None;
        loop {
            let event = reader.get_event().await?;
            match event {
                Event::Token(t) => {
                    token = Some(t);
                    writer_clone.sender.send(Instruction::Login(t)).await.unwrap();
                }
                Event::ChunkUpdate(chunk) => {
                    println!("recieved chunk_update");
                }
                Event::Error(e) => {
                    println!("got error: {:?}", e);
                }
                _ => ()
            }
        }
        Ok::<_, tokio::io::Error>(())
    });

    let mut x: f64 = 0.0;
    loop {
        writer.sender.send(Instruction::Move([x, 100.0, 0.0])).await?;
        x += 1.0;
        sleep(Duration::from_millis(25));
    }
    Ok(())
}
