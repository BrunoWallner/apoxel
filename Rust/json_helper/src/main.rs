use protocol::event::*;
use protocol::*;

fn main() {
    let event = ServerToClient::ChunkUpdate(protocol::chunk::Chunk::new([0, 0, 0]));

    let json = serde_json::to_string(&event).unwrap();

    println!("{}", json);
}
