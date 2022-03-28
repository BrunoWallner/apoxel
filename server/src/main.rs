use tokio::io;
use tokio::net::tcp::{OwnedWriteHalf, OwnedReadHalf};
use tokio::net::{TcpStream, TcpListener};
use tokio::sync::mpsc;

mod config;
use config::CONFIG;

mod broadcast;
mod events;
mod player;
mod chunks;
mod client;

mod logger;

use protocol::{reader, writer};

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> io::Result<()> {
    let _logger = logger::setup().unwrap();
    
    let listener = TcpListener::bind(CONFIG.connection.ip.clone() + ":" + &CONFIG.connection.port.to_string()).await?;
    log::info!("server listenes on: {}", CONFIG.connection.ip.clone() + ":" + &CONFIG.connection.port.to_string());

    let player_handle = player::handle::Handle::init();
    let chunk_handle = chunks::handle::Handle::init(player_handle.clone());

    loop {
        // communication between reader and writer
        let write_broadcast = broadcast::BroadCast::<events::Tcp>::init();
        let (write_tx, write_rx) = mpsc::channel(1024);
        write_broadcast.register(write_tx).await;

        // global handles
        let player_handle = player_handle.clone();
        let chunk_handle = chunk_handle.clone();

        let (socket, _addr) = listener.accept().await?;
        let (r, w) = TcpStream::into_split(socket);
        let mut reader = reader::Reader::new(r);
        let mut writer = writer::Writer::new(w);

        let client_handle = client::handle::Handle::init(
            chunk_handle.clone(),
            player_handle.clone(), 
            write_broadcast.clone(),
        ).await;

        // tcp reader
        tokio::spawn(async move {
            let _ = handle_read(
                &mut reader, 
                client_handle.clone(),
                chunk_handle,
            ).await;

            // when player disconnects logoff
            // extremely important
            client_handle.logoff().await;
        });

        // tcp writer
        tokio::spawn(async move {
            let _ = handle_write(
                &mut writer, 
                write_rx,
            ).await;
        });
    }
}

use protocol::event::Event as ProtocolEvent;
use events::Tcp;

async fn handle_read(
    reader: &mut reader::Reader<OwnedReadHalf>,
    client_handle: client::handle::Handle,
    chunk_handle: chunks::handle::Handle,
) -> io::Result<()> {
    loop {
        let event = reader.get_event().await?;

        match event {
            ProtocolEvent::Register{name} => {
                client_handle.register(name).await;
            }
            ProtocolEvent::Login{token} => {
                client_handle.login(token).await;
            }
            ProtocolEvent::MovePlayer(pos) => {
                client_handle.move_to(pos).await;
            }
            ProtocolEvent::PlaceStructure{pos, structure} => {
                chunk_handle.place_structure(pos, structure).await;
            }
            _ => ()
        }
    }
}

async fn handle_write(
    writer: &mut writer::Writer<OwnedWriteHalf>,
    mut ev_receiver: mpsc::Receiver<events::Tcp>,
) -> io::Result<()> {
    loop {
        match ev_receiver.recv().await {
            Some(e) => match e {
                Tcp::Protocol(ev) => {
                    use ProtocolEvent::*;
                    match ev {
                        Error(e) => {writer.send_event(&Error(e)).await?}
                        ChunkUpdate(chunk) => {
                            writer.send_event(&ChunkUpdate(chunk)).await?
                        }
                        Token(t) => {writer.send_event(&Token(t)).await?}
                        _ => (),
                    }
                }
            }
            None => {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe"));
            }
        }
    }
}