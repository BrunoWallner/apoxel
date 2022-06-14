mod channel;
mod communication;
mod player;
mod chunks;

use bevy::prelude::*;
use protocol::event::prelude::*;
use protocol::chunk::CHUNK_SIZE;
use communication::Communicator;

use chunks::chunk_material::ChunkMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(communication::plugin::CommunicationPlugin)
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .add_plugin(player::PlayerPlugin)
        .add_system(handle_events)
        .add_system(update_player)
        .run();
}

fn update_player(
    player: Query<(&Transform, &player::Player)>,
    communicator: Res<Communicator>,
) {
    for (transform, _) in player.iter().next() {
        let pos = transform.translation;
        let coord = [pos.x as f64, pos.y as f64, pos.z as f64];
        communicator.send_event(Move{coord});
    }
}

fn handle_events(
    mut cmds: Commands,
    communicator: Res<Communicator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
) {
    if let Some(event) = communicator.try_get_event() {
        match event {
            ChunkUpdate(chunk) => {
                let mesh = chunks::mesh::generate(chunk.clone());
                cmds.spawn_bundle(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    transform: Transform::from_xyz(
                        (chunk.coord[0] * CHUNK_SIZE as i64) as f32,
                        (chunk.coord[1] * CHUNK_SIZE as i64) as f32,
                        (chunk.coord[2] * CHUNK_SIZE as i64) as f32
                    ),
                    material: materials.add(ChunkMaterial{}),
                    ..default()
                });

            }
            _ => ()
        }
    }
}
