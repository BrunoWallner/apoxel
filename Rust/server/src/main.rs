mod client;
mod users;
mod logger;
mod channel;
mod tcp;
mod config;

use log::*;
use anyhow::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
pub use config::CONFIG;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> Result<()> {
    let _logger = logger::setup().unwrap();
    
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), CONFIG.connection.port);
    let tcp = tcp::Tcp::init("0.0.0.0:8000").await?;
    info!("server is listening on {}", addr);

    let users = users::Users::init();

    // accepting clients
    loop {
        if let Ok( (rw, addr)) = tcp.accept().await {
            info!("new connection: {}", addr);

            let users = users.clone();

            tokio::spawn(async move {
                client::init(rw, addr, users).await;
            });
        } else {
            warn!("failed to accept client")
        }
    }


    // Ok(())
}
