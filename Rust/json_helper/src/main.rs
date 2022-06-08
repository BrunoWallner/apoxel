use protocol::event::*;
use protocol::*;

fn main() {
    let event = ClientToServer::Move{coord: [0.0, 0.0, 0.0]};

    let json = serde_json::to_string(&event).unwrap();

    println!("{}", json);
}
