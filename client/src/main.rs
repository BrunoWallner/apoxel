mod communication;
mod events;
mod mesh;
mod player;
mod material;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(events::EventHandlePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(material::add)
        .add_startup_system(setup)
        .add_system(event_listener)
        .run();
}

use events::queue::GameEventQueue;
use events::GameEvent;
use protocol::chunk::CHUNK_SIZE;

use std::time::Instant;

fn event_listener(
    mut queue: ResMut<GameEventQueue>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<material::Materials>,
    mut cmds: Commands,
) {
    let start = Instant::now();

    'fetching: loop {
        if let Some(event) = queue.pull() {
            match event {
                GameEvent::SpawnChunkMesh( (mesh, coord) ) => {
                    cmds.spawn().insert_bundle(PbrBundle {
                        mesh: meshes.add(mesh.clone()),
                        material: materials.chunk.clone(),
                        transform: Transform::from_xyz(
                            (coord[0] * CHUNK_SIZE as i64) as f32,
                            (coord[1] * CHUNK_SIZE as i64) as f32,
                            (coord[2] * CHUNK_SIZE as i64) as f32
                        ),
                        ..default()
                    });
                }
            }
        }
        if start.elapsed().as_micros() > 2000 {
            break 'fetching;
        }
    }
}

fn setup(
    mut ambient_light: ResMut<AmbientLight>
) {
    ambient_light.brightness = 1.0;
}
