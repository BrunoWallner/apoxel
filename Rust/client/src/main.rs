mod channel;
mod communication;
mod player;
mod chunks;

use bevy::ecs::entity::Entities;
use bevy::prelude::*;
use protocol::prelude::*;
use protocol::chunk::CHUNK_SIZE;
use protocol::chunk::Chunk;
use communication::Communicator;

use chunks::chunk_material::ChunkMaterial;
use std::collections::BTreeMap;

struct Chunks {
    pub map: BTreeMap<Coord, (Entity, Chunk)>
}
impl Chunks {
    fn new() -> Self {
        Self {
            map: BTreeMap::default()
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Chunks::new())
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
    mut chunks: ResMut<Chunks>,

) {
    if let Some(event) = communicator.try_get_event() {
        match event {
            ChunkLoad (mut chunk) => {
                //  if chunkupdate was faster than chunkload
                if let Some((entity, c)) = chunks.map.get(&chunk.coord) {
                    chunk.merge(c);
                    cmds.entity(*entity).despawn_recursive();
                }
                let mesh = chunks::mesh::generate(chunk.clone());
                let entity = cmds.spawn_bundle(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    transform: Transform::from_xyz(
                        (chunk.coord[0] * CHUNK_SIZE as i64) as f32,
                        (chunk.coord[1] * CHUNK_SIZE as i64) as f32,
                        (chunk.coord[2] * CHUNK_SIZE as i64) as f32
                    ),
                    material: materials.add(ChunkMaterial{}),
                    ..default()
                }).id();
                chunks.map.insert(chunk.coord, (entity, chunk));
            }
            ChunkUpdate (delta) => {
                if let Some((entity, chunk)) = chunks.map.get_mut(&delta.0) {
                    chunk.apply_delta(&delta);
                    cmds.entity(*entity).despawn_recursive();

                    // generate updated mesh
                    let mesh = chunks::mesh::generate(chunk.clone());
                    let new_entity = cmds.spawn_bundle(MaterialMeshBundle {
                        mesh: meshes.add(mesh),
                        transform: Transform::from_xyz(
                            (chunk.coord[0] * CHUNK_SIZE as i64) as f32,
                            (chunk.coord[1] * CHUNK_SIZE as i64) as f32,
                            (chunk.coord[2] * CHUNK_SIZE as i64) as f32
                        ),
                        material: materials.add(ChunkMaterial{}),
                        ..default()
                    }).id();

                    *entity = new_entity;
                } else {
                    let mut chunk = Chunk::new(delta.0);
                    chunk.apply_delta(&delta);

                    // generate updated mesh
                    let mesh = chunks::mesh::generate(chunk.clone());
                    let new_entity = cmds.spawn_bundle(MaterialMeshBundle {
                        mesh: meshes.add(mesh),
                        transform: Transform::from_xyz(
                            (chunk.coord[0] * CHUNK_SIZE as i64) as f32,
                            (chunk.coord[1] * CHUNK_SIZE as i64) as f32,
                            (chunk.coord[2] * CHUNK_SIZE as i64) as f32
                        ),
                        material: materials.add(ChunkMaterial{}),
                        ..default()
                    }).id();

                    chunks.map.insert(delta.0, (new_entity, chunk));
                }
            }
            _ => ()
        }
    }
}
