use tokio::fs::File;
use tokio::io::AsyncReadExt;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub connection: Connection
}
impl Config {
    pub async fn get() -> Result<Self, ()> {
        let mut file = File::open("config.toml").await.unwrap();
        let mut content = vec![];
        file.read_to_end(&mut content).await.unwrap();

        if let Ok(config) = toml::from_slice(&content) {
            return Ok(config)
        } else {
            return Err(());
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    pub ip: String,
    pub port: u16,
}