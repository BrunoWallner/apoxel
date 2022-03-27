mod communication;
use tokio::net::TcpStream;
use protocol::event::Event as TcpEvent;
use futures_lite::future;

use communication::event_queue::Queue;

use bevy::prelude::*;


pub struct Communication {
    pub bridge: communication::bridge::Bridge,
    pub event_queue: Queue,
}

#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("0.0.0.0:8000").await.unwrap();
    let (bridge, event_queue) = communication::init(socket).await;

    bridge.push_tcp(TcpEvent::Register{name: String::from("luca")}).await;

    let com = Communication{bridge, event_queue};

    App::new()
        .insert_resource(com)
        .add_startup_system(setup)
        .add_system(event_pull)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut cmds: Commands,
) {
    cmds.insert_resource(WindowDescriptor {
        title: "Broxel".to_string(), 
        width: 1200.0, 
        height: 800.0, 
        ..default()
    });
}

fn event_pull(
    communication: Res<Communication>
) {
    let ev = future::block_on(communication.event_queue.pull());
    //println!("got event: {:?}", ev);
}
