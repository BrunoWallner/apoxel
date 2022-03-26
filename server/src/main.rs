use tokio::io;
use tokio::net::tcp::{OwnedWriteHalf, OwnedReadHalf};
use tokio::net::{TcpStream, TcpListener};
use tokio::sync::mpsc;

mod config;
use config::CONFIG;

mod broadcaster;
mod events;
mod player;
mod chunks;
mod client_chunk_manager;

mod logger;

use protocol::{reader, writer};

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> io::Result<()> {
    let _logger = logger::setup().unwrap();
    
    let listener = TcpListener::bind(CONFIG.connection.ip.clone() + ":" + &CONFIG.connection.port.to_string()).await?;
    log::info!("server listenes on: {}", CONFIG.connection.ip.clone() + ":" + &CONFIG.connection.port.to_string());

    let players = player::handle::Handle::init();
    let chunks = chunks::handle::Handle::init(players.clone());

    loop {
        // communication between reader and writer
        let write_broadcaster = broadcaster::BroadCaster::<events::Tcp>::init();
        let (write_tx, write_rx) = mpsc::channel(1024);
        write_broadcaster.register(write_tx).await;

        // global handles
        let player_handle = players.clone();
        let chunk_handle = chunks.clone();

        let (socket, _addr) = listener.accept().await?;
        let (r, w) = TcpStream::into_split(socket);
        let mut reader = reader::Reader::new(r);
        let mut writer = writer::Writer::new(w);

        // tcp reader
        let bh_clone = write_broadcaster.clone();
        tokio::spawn(async move {
            let mut player_token: Option<Token> = None;

            let _ = handle_read(
                &mut reader, 
                chunk_handle.clone(),
                player_handle.clone(),
                bh_clone,
                &mut player_token,
            ).await;

            // when player disconnects logoff
            // extremely important
            if let Some(token) = player_token {
                player_handle.logoff(token).await;
            }
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
use protocol::error::Error;
use protocol::Token;
use events::Tcp;

async fn handle_read(
    reader: &mut reader::Reader<OwnedReadHalf>,
    chunk_handle: chunks::handle::Handle,
    player_handle: player::handle::Handle,
    // for communication with the writer
    broadcast_handle: broadcaster::BroadCaster<events::Tcp>,
    player_token: &mut Option<Token>,
) -> io::Result<()> {
    loop {
        let event = reader.get_event().await?;

        match event {
            ProtocolEvent::Register{name} => {
                if let Some(token) = player_handle.register(name).await {
                    broadcast_handle.send(Tcp::Protocol(ProtocolEvent::Token(token))).await;
                } else {
                    broadcast_handle.send(Tcp::Protocol(ProtocolEvent::Error(Error::Register))).await;
                }
            }
            ProtocolEvent::Login{token} => {
                if player_handle.login(token).await {
                    *player_token = Some(token);
                    
                    // client chunk manager init upon login
                    let write_broadcaster_clone = broadcast_handle.clone();
                    let player_handle_clone = player_handle.clone();
                    let chunk_handle_clone = chunk_handle.clone();
                    let (tx, rx) = mpsc::channel(1024);
                    chunk_handle.register_receiver(tx).await;
                    tokio::spawn(async move {
                        client_chunk_manager::init(
                            rx,
                            write_broadcaster_clone,
                            player_handle_clone,
                            chunk_handle_clone,
                            token,
                        ).await;
                    });
                } else {
                    broadcast_handle.send(Tcp::Protocol(ProtocolEvent::Error(Error::Login))).await;
                }
            }
            ProtocolEvent::MovePlayer(pos) => {
                if let Some(token) = player_token {
                    player_handle.move_player(*token, pos).await;
                }
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
                        ChunkUpdate(chunk) => {writer.send_event(&ChunkUpdate(chunk)).await?}
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