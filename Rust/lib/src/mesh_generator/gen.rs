use gdnative::prelude::*;

use super::InternalChunk;
use super::MeshData;
use super::sides::*;

use protocol::chunk::CHUNK_SIZE;

pub fn mesh(chunk: InternalChunk) -> MeshData {
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
                if data[x][y][z] != 0 {
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
    return (coord, verts, uvs, normals, indices);
}

fn flat_to_3d(flat: ByteArray) -> [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] {
    let mut buf = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

    /* 
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
        buf[x][y][z] = group;

        i += 1;
    }
    */
    for (index, value) in flat.to_vec().iter().enumerate() {
        let x = index % CHUNK_SIZE;
        let y = (index / CHUNK_SIZE) % CHUNK_SIZE;
        let z = index / (CHUNK_SIZE * CHUNK_SIZE);

        buf[x][y][z] = *value;
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

fn flatten_index(x: usize, y: usize, z: usize) -> usize {
    x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE * 2 
}

fn get_byte_from_array(array: &ByteArray, index: i32) -> Option<u8> {
    if index < 0 {
        return None;
    }
    if array.len() <= index {
        return None;
    } else {
        return Some(array.get(index));
    }
}