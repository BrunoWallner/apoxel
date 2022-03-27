use bevy::prelude::*;
use crate::communication::Communicator;
use protocol::event::Event as TcpEvent;
use protocol::chunk::CHUNK_SIZE;

use super::communication::GameEvent;
use crate::player::Player;

mod chunk;
mod mesh;

pub struct EventHandlePlugin;

impl Plugin for EventHandlePlugin {
    fn build(&self, app: &mut App) {
        let communicator = Communicator::init("0.0.0.0:8000");
        communicator.event_bridge.push_tcp(TcpEvent::Register{name: String::from("lucawer43")});

        app.insert_resource(communicator)
            .add_system(handle_events)
            .add_system(update_player_pos)
            .add_system(chunk::prepare_mesh_task)
            .add_system(chunk::apply_mesh_task);
    }
}

fn handle_events(
    mut cmds: Commands,
    communicator: Res<Communicator>,
) {
    let evs = communicator.event_queue.pull(5);
    for ev in evs.iter() {
        match ev {
            GameEvent::ChunkUpdate(chunk) => {
                cmds.spawn().insert(chunk::Chunk{chunk: *chunk});
            }
            _ => ()
        }
    }
}

fn update_player_pos(
    mut query: Query<(&Transform, &mut Player)>,
    communicator: Res<Communicator>,
) {
    let mut player_pos = Vec3::ZERO;
    let mut player_moved: bool = false;
    for (transform, mut player) in query.iter_mut().next() {
        player_pos = transform.translation;
        if player.chunks_moved_since_load > 0 {
            player_moved = true;
            player.chunks_moved_since_load = 0;
        }
    } 
    if player_moved {
        communicator.event_bridge.push_tcp(TcpEvent::MovePlayer([player_pos.x as f64, player_pos.y as f64, player_pos.z as f64]));
    }
}