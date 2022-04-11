use std::fs::File;
use std::io::Read;

use serde::{Serialize, Deserialize};

lazy_static! {
    pub static ref CONFIG: Config = {
        Config::get().unwrap()
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub connection: Connection,
    pub chunks: Chunks,
}
impl Config {
    pub fn get() -> Result<Self, ()> {
        let mut file = File::open("config.toml").unwrap();
        let mut content = vec![];
        file.read_to_end(&mut content).unwrap();

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunks {
    pub render_distance: u16,
    pub generations_per_cycle: u16,
}
