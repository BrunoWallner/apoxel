use tokio::io;
use tokio::net::{TcpStream, TcpListener};

mod config;
use config::Config;

mod logger;

mod player;

use protocol::{header::Header, reader, writer};

#[tokio::main]
async fn main() -> io::Result<()> {
    let _logger = logger::setup().unwrap();

    let config = Config::get().await.unwrap();
    
    let listener = TcpListener::bind(config.connection.ip.clone() + ":" + &config.connection.port.to_string()).await?;
    log::info!("server listenes on: {}", config.connection.ip + ":" + &config.connection.port.to_string());

    // accept and handle incoming connections
    let players = player::handler::Handler::init();
    loop {
        let player_handler = players.clone();
        let (mut socket, addr) = listener.accept().await?;

        tokio::spawn(async move {
            let result = handle(
                &mut socket, 
                player_handler
            ).await;

            match result {
                Ok(_) => {},
                Err(e) => match e.kind() {
                    io::ErrorKind::BrokenPipe => {
                        log::info!("{} lost connection", addr)
                    }
                    _ => (),
                }
            }
        });
    }
}

async fn handle(
    socket: &mut TcpStream,
    player_handler: player::handler::Handler,

) -> io::Result<()> {
    let (r, w) = TcpStream::split(socket);

    let mut reader = reader::Reader::new(r);
    let mut writer = writer::Writer::new(w);

    // handle infinite amount instructions
    loop {
        let header: Header = reader.get_header().await?;

        match header {
            Header::Register => {
                let name = reader.get_string().await?;
                if let Some(token) = player_handler.register(name).await {
                    writer.send_token(&token).await?;
                } else {
                    writer.send_error(&protocol::error::Error::Register).await?;
                }
            }
            _ => ()
        }
    }
}