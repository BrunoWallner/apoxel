use std::fs::File;
use std::io::Read;

use serde::{Serialize, Deserialize};

// delays startup time
lazy_static! {
    pub static ref CONFIG: Config = {
        match Config::get() {
            Ok(c) => c,
            Err(e) => {
                log::warn!("invalid config: {}", e);
                std::process::exit(1);
            }
        }
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub connection: Connection,
    pub chunks: Chunks,
}
impl Config {
    // delays startup time
    pub fn get() -> Result<Self, toml::de::Error> {
        let mut file = File::open("config.toml").unwrap();
        let mut content = vec![];
        file.read_to_end(&mut content).unwrap();

        toml::from_slice(&content)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunks {
    pub render_distance: u16,
    pub generations_per_cycle: u16,
}
