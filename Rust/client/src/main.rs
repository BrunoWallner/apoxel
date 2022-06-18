mod channel;
mod communication;
mod player;
mod chunks;

use bevy::prelude::*;
use protocol::prelude::*;
use protocol::chunk::CHUNK_SIZE;
use protocol::chunk::Chunk;
use communication::Communicator;

use chunks::chunk_material::ChunkMaterial;
use std::collections::BTreeMap;

use bevy_mod_picking::*;

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

struct Tick(u64);

fn main() {
    App::new()
        .insert_resource(Chunks::new())
        .insert_resource(Tick(0))
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(communication::plugin::CommunicationPlugin)
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .add_plugin(player::PlayerPlugin)
        .add_system(handle_events)
        .add_system(update_player)
        // .add_system(block_placing)
        .run();
}

// fn block_placing(
//     mut cmds: Commands,
//     mut events: EventReader<PickingEvent>,
//     communicator: Res<Communicator>,
// ) {
//     for event in events.iter() {
//         match event {
//             PickingEvent::Clicked(entity) => {
//                 cmds.entity(*entity).
//             }
//             _ => ()
//         }
//     }
// }

fn update_player(
    player: Query<(&Transform, &player::Player)>,
    communicator: Res<Communicator>,
    input: Res<Input<KeyCode>>,
    mut tick: ResMut<Tick>,
) {
    for (transform, _) in player.iter().next() {
        tick.0 += 1;
        if tick.0 % 1 == 0 {
            let pos = transform.translation;
            let coord = [pos.x as f64, pos.y as f64, pos.z as f64];
            communicator.send_event(Move{coord});
    
            if input.pressed(KeyCode::P) {
                let mut structure = Structure::new([10, 10, 10]);
                for x in 0..10 {
                    for y in 0..10 {
                        for z in 0..10 {
                            let coord = [x, y, z];
                            if protocol::calculate_chunk_distance(&coord, &[5, 5, 5]) < 5 {
                                structure.place([x as usize, y as usize, z as usize], Block::Air);
                            }
                        }
                    }
                }
                let coord = [pos.x as i64, pos.y as i64 - 8, pos.z as i64];
                communicator.send_event(PlaceStructure{coord, structure});
            }
        }
    }
}

fn handle_events(
    mut cmds: Commands,
    communicator: Res<Communicator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut chunks: ResMut<Chunks>,
) {
    while let Some(event) = communicator.try_get_event() {
        match event {
            ChunkLoads (mut chunkloads) => {
                for chunk in chunkloads.iter_mut() {
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
                    })
                    .insert_bundle(PickableBundle::default())
                    .id();
                    chunks.map.insert(chunk.coord, (entity, chunk.clone()));
                }
            }
            ChunkUpdates (deltas) => {
                for delta in deltas.iter() {
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
                        })                    
                        .insert_bundle(PickableBundle::default())
                        .id();
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
                        })
                        .insert_bundle(PickableBundle::default())
                        .id();
    
                        chunks.map.insert(delta.0, (new_entity, chunk));
                    }
                }
            }
            ChunkUnloads (coords) => {
                for coord in coords.iter() {
                    if let Some((entity, _chunk)) = chunks.map.get(coord) {
                        cmds.entity(*entity).despawn_recursive();
                        chunks.map.remove(coord);
                    }
                }
            }
            _ => ()
        }
    }
}
