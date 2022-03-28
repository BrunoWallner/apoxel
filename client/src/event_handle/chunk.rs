use std::collections::HashMap;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::Task;

use futures_lite::future;

use protocol::{
    Coord,
    chunk::{
        CHUNK_SIZE,
        ChunkData,
    }
};

pub struct VoxelMap {
    pub map: HashMap<Coord, ChunkData>
}

#[derive(Component)]
pub struct Chunk{
    pub coord: Coord
}

pub fn unloader(
    mut cmds: Commands,
    mut query: QuerySet<(
        QueryState<(Entity, &Chunk)>,
        QueryState<(&Transform, &mut super::Player)>,
    )>,
    mut voxel_map: ResMut<VoxelMap>,
) {
    let mut player_pos = Vec3::ZERO;
    let mut player_moved: bool = false;
    for (transform, mut player) in query.q1().iter_mut().next() {
        player_pos = transform.translation;
    }
    for (entity, chunk) in query.q0().iter() {
        let coord = chunk.coord;
        let chunk_pos = protocol::chunk::get_chunk_coords(&[player_pos[0] as i64, player_pos[1] as i64, player_pos[2] as i64]).0;

        let distance = (( 
            (coord[0] - chunk_pos[0]).pow(2) +
            (coord[1] - chunk_pos[1]).pow(2) +
            (coord[2] - chunk_pos[2]).pow(2)
        ) as f64).sqrt() as i64;

        if distance > 10 {
            cmds.entity(entity).despawn();
            voxel_map.map.remove(&coord);
        }
    }
}

#[derive(Component)]
pub struct MeshTask {
    pub task: Task<Mesh>
}

pub fn prepare_mesh_task(
    mut cmds: Commands,
    query: Query<(Entity, &Chunk), Added<Chunk>>,
    voxel_map: Res<VoxelMap>,
    pool: Res<AsyncComputeTaskPool>,
) {
    for (entity, chunk) in query.iter() {
        let coord = chunk.coord.clone();
        if let Some(data) = voxel_map.map.get(&coord) {
            let data = data.clone();
            let task = pool.spawn(async move {
                let mesh = super::mesh::generate(protocol::chunk::Chunk {
                    data: data,
                    coord: coord,
                });
        
                mesh
            });
    
            cmds.entity(entity).insert(MeshTask{task});
        }
    }
}

pub fn apply_mesh_task(
    mut cmds: Commands,
    mut query: Query<(Entity, &Chunk, &mut MeshTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<crate::material::Materials>,
) {
    for (entity, chunk, mut task) in query.iter_mut() {
        if let Some(mesh) = future::block_on(future::poll_once(&mut task.task)) {
            cmds.entity(entity).insert_bundle(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.chunk.clone(),
                transform: Transform::from_xyz(
                    (chunk.coord[0] * CHUNK_SIZE as i64) as f32,
                    (chunk.coord[1] * CHUNK_SIZE as i64) as f32,
                    (chunk.coord[2] * CHUNK_SIZE as i64) as f32
                ),
                ..default()
            });
            cmds.entity(entity).remove::<MeshTask>();
        }
    }
}