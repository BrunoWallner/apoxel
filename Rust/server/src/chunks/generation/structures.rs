use protocol::chunk::Structure;

use std::io::Read;
use std::fs;


lazy_static! {
    pub static ref TREE: Structure = {
        load_struct("tree.dat")
    };
}

fn load_struct(path: &str) -> Structure {
    let path = String::from("./models/") + path;

    log::info!("path: {}", path);

    let mut file = fs::File::open(path).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    bincode::deserialize(&buffer).unwrap()
}