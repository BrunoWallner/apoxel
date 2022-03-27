use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::Task;

use futures_lite::future;

use protocol::chunk::CHUNK_SIZE;

#[derive(Component)]
pub struct Chunk{
    pub chunk: protocol::chunk::Chunk,
}

#[derive(Component)]
pub struct MeshTask {
    pub task: Task<Mesh>
}

pub fn prepare_mesh_task(
    mut cmds: Commands,
    mut query: Query<(Entity, &Chunk), Added<Chunk>>,
    pool: Res<AsyncComputeTaskPool>,
) {
    for (entity, chunk) in query.iter() {
        let chunk = chunk.chunk.clone();
        let task = pool.spawn(async move {
            let mesh = super::mesh::generate(chunk);
    
            mesh
        });

        cmds.entity(entity).insert(MeshTask{task});
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
                    (chunk.chunk.coord[0] * CHUNK_SIZE as i64) as f32,
                    (chunk.chunk.coord[1] * CHUNK_SIZE as i64) as f32,
                    (chunk.chunk.coord[2] * CHUNK_SIZE as i64) as f32
                ),
                ..default()
            });
            cmds.entity(entity).remove::<MeshTask>();
        }
    }
}