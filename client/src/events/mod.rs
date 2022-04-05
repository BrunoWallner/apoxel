pub mod queue;
pub mod process;

use bevy::prelude::*;

use crate::communication::Communicator;

pub struct EventHandlePlugin;

impl Plugin for EventHandlePlugin {
    fn build(&self, app: &mut App) {
        // make sure not to clone game_rx and thus handle events multiple times
        let communicator = Communicator::init("0.0.0.0:8000");

        app.insert_resource(communicator);
    }
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    SpawnChunkMesh(Mesh),
}
