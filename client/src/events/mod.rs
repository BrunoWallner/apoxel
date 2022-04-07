pub mod queue;
pub mod process;

use bevy::prelude::*;

use crate::communication::Communicator;

use protocol::event::Event as TcpEvent;
use protocol::Coord;

pub struct EventHandlePlugin;

impl Plugin for EventHandlePlugin {
    fn build(&self, app: &mut App) {
        // make sure not to clone game_rx and thus handle events multiple times
        let communicator = Communicator::init("0.0.0.0:8000");
        communicator.event_bridge.push_tcp(TcpEvent::Register{name: String::from("lucawer43")});

        let game_event_queue = queue::GameEventQueue::init();

        // init processor
        process::init(communicator.communication_rx.clone(), game_event_queue.clone());

        app.insert_resource(communicator)
            .insert_resource(game_event_queue);
    }
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    SpawnChunkMesh( (Mesh, Coord) ),
}
