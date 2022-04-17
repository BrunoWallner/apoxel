use gdnative::prelude::*;

use super::InternalChunk;
use super::sides::*;

use protocol::chunk::CHUNK_SIZE;

use gdnative::api::{Mesh, ArrayMesh, CollisionShape, StaticBody, MeshInstance};

pub fn mesh(chunk: InternalChunk) -> Option<Ref<Spatial>> {
    let coord = chunk.0;

    let data = chunk.1;
    let data = flat_to_3d(data);

    let mut verts = Vector3Array::new();
    let mut uvs = Vector2Array::new();
    let mut normals = Vector3Array::new();
    let mut indices: PoolArray<i32> = PoolArray::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if data[x][y][z].0 != 0 {
                    if check_left(x, y, z, &data) {
                        let side = LEFT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(
                            &mut verts,
                            &mut uvs,
                            &mut normals,
                            &mut indices,
                            &side
                        );
                    }
                    if check_right(x, y, z, &data) {
                        let side = RIGHT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(
                            &mut verts,
                            &mut uvs,
                            &mut normals,
                            &mut indices,
                            &side
                        );
                    }
                    if check_front(x, y, z, &data) {
                        let side = FRONT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(
                            &mut verts,
                            &mut uvs,
                            &mut normals,
                            &mut indices,
                            &side
                        );
                    }
                    if check_back(x, y, z, &data) {
                        let side = BACK.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(
                            &mut verts,
                            &mut uvs,
                            &mut normals,
                            &mut indices,
                            &side
                        );
                    }
                    if check_top(x, y, z, &data) {
                        let side = TOP.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(
                            &mut verts,
                            &mut uvs,
                            &mut normals,
                            &mut indices,
                            &side
                        );
                    }
                    if check_bottom(x, y, z, &data) {
                        let side = BOTTOM.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(
                            &mut verts,
                            &mut uvs,
                            &mut normals,
                            &mut indices,
                            &side
                        );
                    }
                }
            }
        }
    }
    if !verts.is_empty() {
        //return (coord, verts, uvs, normals, indices);
        let arr = VariantArray::new();
        arr.resize(Mesh::ARRAY_MAX as i32);

        arr.set(Mesh::ARRAY_VERTEX as i32, verts);
        arr.set(Mesh::ARRAY_TEX_UV as i32, uvs);
        arr.set(Mesh::ARRAY_NORMAL as i32, normals);
        arr.set(Mesh::ARRAY_INDEX as i32, indices);

        let array_mesh = ArrayMesh::new();
        array_mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, arr.into_shared(), VariantArray::new().into_shared(), 2194432);
        let mesh_collision_shape = array_mesh.create_trimesh_shape();

        let collision_shape = CollisionShape::new();
        collision_shape.set_shape(mesh_collision_shape.unwrap());

        let static_body = StaticBody::new();
        static_body.add_child(collision_shape, false);

        let mesh_instance = MeshInstance::new();
        mesh_instance.set_mesh(array_mesh);
        mesh_instance.add_child(static_body, false);

        let spatial = Spatial::new();
        spatial.translate(Vector3{x: coord.get(0) as f32, y: coord.get(1) as f32, z: coord.get(2) as f32});
        spatial.add_child(mesh_instance, false);

        Some(spatial.into_shared())
    } else {
        None
    }
}

fn flat_to_3d(flat: ByteArray) -> [[[(u8, u8); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] {
    let mut buf = [[[(0, 0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
    let mut iterator = flat.to_vec().into_iter();
    
    let mut i: usize = 0;
    'converting: loop {
        let group = match iterator.next() {
            Some(v) => {v},
            None => {break 'converting}
        };

        let block = match iterator.next() {
            Some(v) => {v},
            None => {break 'converting}
        };

        let x = i % CHUNK_SIZE;
        let y = (i / CHUNK_SIZE) % CHUNK_SIZE;
        let z = i / (CHUNK_SIZE * CHUNK_SIZE);
        buf[x][y][z] = (group, block);

        i += 1;
    }

    buf
}

fn push_side(
    verts: &mut Vector3Array, 
    uvs: &mut Vector2Array, 
    normals: &mut Vector3Array,
    indices: &mut PoolArray<i32>,
    side: &super::sides::Side,
) {
    verts.append(&PoolArray::from_slice(&side.verts));
    for _ in 0..4 {
        normals.push(side.normal);
    }
    let offset = verts.len() as i32 - 4;
    for i in 0..6 {
        let index = side.indices.get(i);
        indices.push(index + offset);
    }
    for _ in 0..4 {
        uvs.push(Vector2{x: 0.0, y: 0.0});
    }
}