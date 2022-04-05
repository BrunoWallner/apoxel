mod mesh;

use crossbeam::channel;
use protocol::chunk::Chunk;
use crate::event_handle::Chunk as GameChunk;
use super::event_handle::chunk::ChunkMap;

use bevy::prelude::*;

pub(super) struct MeshSender {
    sender: channel::Sender<QueueEvent>
}

#[derive(Component)]
pub struct LoadedChunk {
    pub coord: Coord,
}

pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        let (queue_tx, queue_rx) = channel::unbounded();
        let mesh_sender = MeshSender {sender: queue_tx.clone()};
        init_mesh_queue(queue_tx, queue_rx);

        app
            .insert_resource(mesh_sender)
            .add_system(send_mesh_system)
            .add_system(apply_mesh_system);
    }
}

fn send_mesh_system(
    mesh_sender: Res<MeshSender>,
    chunks: Query<&GameChunk, Added<GameChunk>>,
    chunk_map: Res<ChunkMap>,
) {
    for chunk in chunks.iter() {
        let coord = chunk.0;
        if let Some(chunk) = chunk_map.0.get(&coord) {
            if let Some(chunk) = chunk.1 {
                mesh_sender.sender.send(QueueEvent::PushChunk(chunk)).unwrap();
                // TODO delete chunk content to save memory
            }
        }
    }
}

use protocol::chunk::CHUNK_SIZE;

fn apply_mesh_system(
    mut cmds: Commands,
    mesh_sender: Res<MeshSender>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<crate::material::Materials>,
    chunk_map: Res<ChunkMap>,
) {
    let (tx, rx) = channel::unbounded();
    mesh_sender.sender.send(QueueEvent::PullMesh(tx)).unwrap();

    for (mesh, coord) in rx.recv().unwrap() {
        if let Some(map_content) = chunk_map.0.get(&coord) {
            let entity = map_content.0;
            cmds.entity(entity).insert_bundle(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.chunk.clone(),
                transform: Transform::from_xyz(
                    (coord[0] * CHUNK_SIZE as i64) as f32,
                    (coord[1] * CHUNK_SIZE as i64) as f32,
                    (coord[2] * CHUNK_SIZE as i64) as f32
                ),
                ..default()
            })
            .insert(LoadedChunk{coord});
        }
    }
}


use std::thread;
use protocol::Coord;

enum QueueEvent {
    PushChunk(Chunk),
    PushMesh((Mesh, Coord)),
    PullMesh(channel::Sender<Vec<(Mesh, Coord)>>)
}

fn init_mesh_queue(tx: channel::Sender<QueueEvent>, rx: channel::Receiver<QueueEvent>) {
    thread::spawn(move || {
        let mut mesh_queue: Vec< (Mesh, Coord) > = Vec::new();

        loop {
            match rx.recv().unwrap() {
                QueueEvent::PushChunk(chunk) => {
                    let tx = tx.clone();
                    thread::spawn(move || {
                        let coord = chunk.coord;
                        let mesh = mesh::generate(chunk);
                        tx.send(QueueEvent::PushMesh( (mesh, coord) )).unwrap();
                    });
                    mesh_queue.sort_unstable_by_key(|key| {
                        let coord = key.1;
                        ((coord[0].pow(2) + coord[1].pow(2) + coord[2].pow(2) )as f32).sqrt() as i64 * (-1)
                    })
                }
                QueueEvent::PushMesh( (mesh, coord) ) => {
                    mesh_queue.push( (mesh, coord) )
                }
                QueueEvent::PullMesh(sender) => {
                    sender.send(mesh_queue.drain(..).as_slice().to_vec()).unwrap();
                }
            }
        }
    });
}