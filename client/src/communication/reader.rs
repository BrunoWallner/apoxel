use super::bridge::Bridge;
use protocol::{event::Event, reader::Reader};
use tokio::net::tcp::OwnedReadHalf;
use super::GameEvent;

#[allow(unreachable_code)]
pub fn init(
    bridge: Bridge,
    mut reader: Reader<OwnedReadHalf>,
) {
    tokio::spawn(async move {
        loop {
            let event = reader.get_event().await?;
            match event {
                // when successfull register
                Event::Token(t) => {
                    bridge.push_game(GameEvent::Token(t)).await;
                    bridge.push_tcp(Event::Login{token: t}).await;
                    // TODO: save token
                }
                Event::ChunkUpdate(_chunk) => {
                }
                Event::Error(e) => {
                    println!("got error: {:?}", e);
                }
                _ => ()
            }
        }
        Ok::<_, tokio::io::Error>(())
    });
}