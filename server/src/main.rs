use tokio::io;
use tokio::net::tcp::{OwnedWriteHalf, OwnedReadHalf};
use tokio::net::{TcpStream, TcpListener};
use tokio::sync::mpsc;

mod config;
use config::Config;

mod event;
mod player;
mod chunks;

mod logger;

use protocol::{header::Header, reader, writer};

#[tokio::main]
async fn main() -> io::Result<()> {
    let _logger = logger::setup().unwrap();

    let config = Config::get().await.unwrap();
    
    let listener = TcpListener::bind(config.connection.ip.clone() + ":" + &config.connection.port.to_string()).await?;
    log::info!("server listenes on: {}", config.connection.ip + ":" + &config.connection.port.to_string());

    let players = player::handle::Handle::init();
    let chunks = chunks::handle::Handle::init(players.clone());

    loop {
        // communication between reader and writer
        let write_broadcaster = event::broadcaster::BroadCaster::init();
        let (write_tx, write_rx) = mpsc::channel(1024);
        write_broadcaster.register(write_tx).await;

        let player_handler = players.clone();
        let broadcast_handler = write_broadcaster.clone();

        let (socket, _addr) = listener.accept().await?;
        let (r, w) = TcpStream::into_split(socket);
        let mut reader = reader::Reader::new(r);
        let mut writer = writer::Writer::new(w);

        tokio::spawn(async move {
            let _ = handle_read(
                &mut reader, 
                player_handler,
                broadcast_handler,
            ).await;
        });

        tokio::spawn(async move {
            let _ = handle_write(
                &mut writer, 
                write_rx,
            ).await;
        });
    }
}

use event::Event;
use protocol::event::Event as ExternalEvent;

async fn handle_read(
    reader: &mut reader::Reader<OwnedReadHalf>,
    player_handler: player::handle::Handle,
    broadcast_handler: event::broadcaster::BroadCaster,
) -> io::Result<()> {
    // handle infinite amount of instructions
    loop {
        let header: Header = reader.get_header().await?;

        match header {
            Header::Register => {
                let name = reader.get_string().await?;
                if let Some(token) = player_handler.register(name).await {
                    broadcast_handler.send(Event::External(ExternalEvent::Token(token))).await;
                } else {
                    broadcast_handler.send(Event::External(ExternalEvent::Error(protocol::error::Error::Register))).await;
                }
            }
            _ => ()
        }
    }
}

async fn handle_write(
    writer: &mut writer::Writer<OwnedWriteHalf>,
    mut ev_receiver: mpsc::Receiver<Event>,
) -> io::Result<()> {
    loop {
        match ev_receiver.recv().await {
            Some(e) => match e {
                Event::External(ev) => {
                    use ExternalEvent::*;
                    match ev {
                        Error(e) => {writer.send_event(&Error(e)).await?}
                        Token(t) => {writer.send_event(&Token(t)).await?}
                        _ => (),
                    }
                }
                _ => (),
            }
            None => {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe"));
            }
        }
    }
}