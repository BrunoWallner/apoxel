mod client;
mod users;
mod chunks;
mod logger;
mod channel;
mod tcp;
mod config;
mod queque;

use log::*;
use anyhow::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
pub use config::CONFIG;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup().unwrap();
    
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), CONFIG.connection.port);
    let tcp = tcp::Tcp::init(addr).await?;
    info!("server is listening on {}", addr);

    let users = users::Users::init();
    let chunk_handle = chunks::ChunkHandle::init();

    // accepting clients
    loop {
        if let Ok( ((read_queque, write_queque), addr)) = tcp.accept().await {
            let users = users.clone();
            let chunk_handle = chunk_handle.clone();

            tokio::spawn(async move {
                client::init(read_queque, write_queque, addr, users, chunk_handle).await;
            });
        }
    }


    // Ok(())
}
