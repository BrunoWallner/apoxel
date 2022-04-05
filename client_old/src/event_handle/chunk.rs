use std::collections::HashMap;

use bevy::prelude::*;
use protocol::{Coord, chunk::Chunk};

pub struct ChunkMap(
    pub HashMap<Coord, (Entity, Option<Chunk>)>
);

pub struct ChunkUnloadCheck(pub usize);

pub fn unloader(
    mut cmds: Commands,
    player: Query<&Transform, With<crate::player::Player>>,
    mut chunk_map: ResMut<ChunkMap>,
    mut unload_check: ResMut<ChunkUnloadCheck>,
) {
    let mut player_pos = Vec3::ZERO;
    for transform in player.iter().next() {
        player_pos = transform.translation;
    }

    let chunk_len = chunk_map.0.len();
    if unload_check.0 >= chunk_len {
        unload_check.0 = 0;
    }
    let check_start = unload_check.0;
    let check_end = if chunk_len > check_start + 100 {
        check_start + 100
    } else {
        chunk_len
    };


    if chunk_len > 0 {
        let mut map: Vec<(Coord, Entity)> = chunk_map.0
            .iter()
            .map(|x| (*x.0, (x.1).0) )
            .collect();

        let chunk_pos = protocol::chunk::get_chunk_coords(&[player_pos[0] as i64, player_pos[1] as i64, player_pos[2] as i64]).0;

        map.sort_unstable_by_key(|key| {
            let coord = key.0;
            let distance = (( 
                (coord[0] - chunk_pos[0]).pow(2) +
                (coord[1] - chunk_pos[1]).pow(2) +
                (coord[2] - chunk_pos[2]).pow(2)
            ) as f64).sqrt() as i64;
            distance
        });

        for (coord, entity) in map[check_start..check_end].iter() {        
            let distance = (( 
                (coord[0] - chunk_pos[0]).pow(2) +
                (coord[1] - chunk_pos[1]).pow(2) +
                (coord[2] - chunk_pos[2]).pow(2)
            ) as f64).sqrt() as i64;

            if distance > 10 {
                cmds.entity(*entity).despawn();
                chunk_map.0.remove(coord);
            }
            unload_check.0 += 1;
        }
    }
}

// #[derive(Component)]
// pub struct MeshTask {
//     pub task: Task<Mesh>
// }

// pub fn prepare_mesh_task(
//     mut cmds: Commands,
//     query: Query<(Entity, &Chunk), Added<Chunk>>,
//     voxel_map: Res<VoxelMap>,
//     pool: Res<AsyncComputeTaskPool>,
// ) {
//     for (entity, chunk) in query.iter() {
//         let coord = chunk.coord.clone();
//         if let Some(data) = voxel_map.map.get(&coord) {
//             let data = data.clone();
//             let task = pool.spawn(async move {
//                 let mesh = super::mesh::generate(protocol::chunk::Chunk {
//                     data: data,
//                     coord: coord,
//                 });
        
//                 mesh
//             });
    
//             cmds.entity(entity).insert(MeshTask{task});
//         }
//     }
// }

// pub fn apply_mesh_task(
//     mut cmds: Commands,
//     mut query: Query<(Entity, &Chunk, &mut MeshTask)>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     materials: Res<crate::material::Materials>,
// ) {
//     for (entity, chunk, mut task) in query.iter_mut() {
//         if let Some(mesh) = future::block_on(future::poll_once(&mut task.task)) {
//             cmds.entity(entity).insert_bundle(PbrBundle {
//                 mesh: meshes.add(mesh),
//                 material: materials.chunk.clone(),
//                 transform: Transform::from_xyz(
//                     (chunk.coord[0] * CHUNK_SIZE as i64) as f32,
//                     (chunk.coord[1] * CHUNK_SIZE as i64) as f32,
//                     (chunk.coord[2] * CHUNK_SIZE as i64) as f32
//                 ),
//                 ..default()
//             });
//             cmds.entity(entity).remove::<MeshTask>();
//         }
//     }
// }