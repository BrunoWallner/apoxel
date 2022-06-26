mod communication;
mod player;
mod chunks;

use bevy::prelude::*;
use bevy::diagnostic::*;
use protocol::prelude::*;
use protocol::chunk::CHUNK_SIZE;
use communication::Communicator;
use chunks::communication::ExternalEvent;

use chunks::chunk_material::ChunkMaterial;
use std::collections::BTreeMap;
use std::time::Instant;

struct ChunkMap {
    pub map: BTreeMap<Coord, Entity>
}
impl ChunkMap {
    fn new() -> Self {
        Self {
            map: BTreeMap::default()
        }
    }
}

struct Tick(u64);

fn main() {
    App::new()
        .insert_resource(ChunkMap::new())
        .insert_resource(chunks::communication::ChunkCommunicator::new())
        .insert_resource(Tick(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(communication::plugin::CommunicationPlugin)
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .add_system(handle_events)
        .add_system(update_player)
        .add_system(handle_chunks)
        // .add_system(block_placing)
        .run();
}

fn setup(
    mut clear_color: ResMut<ClearColor>,
    mut diagnostics: ResMut<Diagnostics>
) {
    clear_color.0 = Color::BLACK;
    diagnostics.add(Diagnostic::new(
        DiagnosticId::from_u128(1),
        "chunks",
        128,
    ));
}

fn update_player(
    player: Query<(&Transform, &player::Player)>,
    communicator: Res<Communicator>,
    input: Res<Input<MouseButton>>,
    mut tick: ResMut<Tick>,
) {
    for (transform, _) in player.iter().next() {
        tick.0 += 1;
        if tick.0 % 1 == 0 {
            let pos = transform.translation;
            let coord = [pos.x as f64, pos.y as f64, pos.z as f64];
            communicator.send_event(Move{coord});

            let size: i64 = 24;
            if input.just_pressed(MouseButton::Left) {
                let mut structure = Structure::new([size as usize, size as usize, size as usize]);
                for x in 0..size {
                    for y in 0..size {
                        for z in 0..size {
                            let coord = [x, y, z];
                            if protocol::calculate_chunk_distance(&coord, &[size / 2, size / 2, size / 2]) < size / 2 {
                                structure.place([x as usize, y as usize, z as usize], Block::Air);
                            }
                        }
                    }
                }
                let coord = [pos.x as i64, pos.y as i64 - size / 2, pos.z as i64];
                communicator.send_event(PlaceStructure{coord, structure});
            } else if input.just_pressed(MouseButton::Right) {
                let mut structure = Structure::new([size as usize, size as usize, size as usize]);
                for x in 0..size {
                    for y in 0..size {
                        for z in 0..size {
                            let coord = [x, y, z];
                            if protocol::calculate_chunk_distance(&coord, &[size / 2, size / 2, size / 2]) < size / 2 {
                                structure.place([x as usize, y as usize, z as usize], Block::AppleTreeWood);
                            }
                        }
                    }
                }
                let coord = [pos.x as i64, pos.y as i64 - size / 2, pos.z as i64];
                communicator.send_event(PlaceStructure{coord, structure});
            }
        }
    }
}



fn handle_chunks(
    mut cmds: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    chunk_communicator: Res<chunks::communication::ChunkCommunicator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut diagnostics: ResMut<Diagnostics>
) {
    let now = Instant::now();
    for _ in 0..5 {
        if let Some(event) = chunk_communicator.try_get() {
            match event {
                ExternalEvent::Load((coord, mesh)) => {
                    if let Some(entity) = chunk_map.map.get(&coord) {
                        cmds.entity(*entity).despawn_recursive();
                    }
                    let entity = cmds.spawn_bundle(MaterialMeshBundle {
                        mesh: meshes.add(mesh),
                        transform: Transform::from_xyz(
                            (coord[0] * CHUNK_SIZE as i64) as f32,
                            (coord[1] * CHUNK_SIZE as i64) as f32,
                            (coord[2] * CHUNK_SIZE as i64) as f32
                        ),
                        material: materials.add(ChunkMaterial{}),
                        ..default()
                    }).id();
                    chunk_map.map.insert(coord, entity);
                },
                ExternalEvent::Unload(coords) => {
                    for coord in coords.iter() {
                        if let Some(entity) = chunk_map.map.get(coord) {
                            cmds.entity(*entity).despawn_recursive();
                            chunk_map.map.remove(coord);
                        }
                    }
                }
            }
        }
    }
    diagnostics.add_measurement(DiagnosticId::from_u128(1), now.elapsed().as_micros() as f64);
}

fn handle_events(
    communicator: Res<Communicator>,
    chunk_communicator: Res<chunks::communication::ChunkCommunicator>,
) {
    for _ in 0..10 {
        if let Some(event) = communicator.try_get_event() {
            match event {
                ChunkLoads (chunkloads) => {
                    chunk_communicator.load(chunkloads);
                }
                ChunkUpdates (deltas) => {
                    chunk_communicator.update(deltas);
                }
                ChunkUnloads (coords) => {
                    chunk_communicator.unload(coords);
                }
                _ => ()
            }
        }
    }
}
