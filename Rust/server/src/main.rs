mod logger;
mod channel;
mod tcp;


use log::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _logger = logger::setup().unwrap();
    let tcp = tcp::Tcp::init("0.0.0.0:8000").await?;
    
    Ok(())
}
