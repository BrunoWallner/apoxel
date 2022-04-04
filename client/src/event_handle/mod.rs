use bevy::prelude::*;
use crate::communication::Communicator;
use protocol::event::Event as TcpEvent;
use protocol::Coord;

use super::communication::GameEvent;
use crate::player::Player;
use std::collections::HashMap;

pub mod chunk;

#[derive(Component)]
pub struct Chunk(pub Coord);

pub struct EventHandlePlugin;


impl Plugin for EventHandlePlugin {
    fn build(&self, app: &mut App) {
        let communicator = Communicator::init("0.0.0.0:8000");
        communicator.event_bridge.push_tcp(TcpEvent::Register{name: String::from("lucawer43")});

        app.insert_resource(communicator)
            .insert_resource(chunk::ChunkUnloadCheck(0))
            .insert_resource(chunk::ChunkMap(HashMap::default()))
            .add_system(chunk::unloader)
            .add_system(handle_events)
            .add_system(update_player_pos);
    }
}

fn handle_events(
    mut cmds: Commands,
    communicator: Res<Communicator>,
    mut chunk_map: ResMut<chunk::ChunkMap>,
) {
    let evs = communicator.event_queue.pull(15);
    for ev in evs.iter() {  
         match ev {
            GameEvent::ChunkUpdate(chunk) => {
                let entity = cmds.spawn().insert(Chunk(chunk.coord)).id();
                chunk_map.0.insert(chunk.coord, (entity, Some(*chunk)));
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