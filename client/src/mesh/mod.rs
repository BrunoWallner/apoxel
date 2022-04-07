mod sides;
use sides::push_uvs;

use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::mesh::Indices;

use protocol::chunk::{CHUNK_SIZE, Chunk};

// ca. 150 - 1200 µs
pub fn generate(
    chunk: Chunk,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let v_length = 8 * CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(v_length);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(v_length);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(v_length);

    let mut indices: Vec<u32> = Vec::with_capacity(v_length);

    for x1 in 0..CHUNK_SIZE {
        for y1 in 0..CHUNK_SIZE {
            for z1 in 0..CHUNK_SIZE {

                let x: f32 = x1 as f32;
                let y: f32 = y1 as f32;
                let z: f32= z1 as f32;

                let block_category = chunk.data[x1][y1][z1].to_category();
                
                // air
                if block_category.0 != 0 { 
                    if check_left(&chunk, x1, y1, z1) {
                        sides::left(x, y, z, &mut positions, &mut normals, &mut indices);
                        push_uvs(&mut uvs, block_category);
                    }
                    if check_right(&chunk, x1, y1, z1) {
                        sides::right(x, y, z, &mut positions, &mut normals, &mut indices);
                        push_uvs(&mut uvs, block_category);
                    }
                    if check_back(&chunk, x1, y1, z1) {
                        sides::back(x, y, z, &mut positions, &mut normals, &mut indices);
                        push_uvs(&mut uvs, block_category);
                    }
                    if check_front(&chunk, x1, y1, z1) {
                        sides::front(x, y, z, &mut positions, &mut normals, &mut indices);
                        push_uvs(&mut uvs, block_category);
                    }
                    if check_top(&chunk, x1, y1, z1) {
                        sides::top(x, y, z, &mut positions, &mut normals, &mut indices);
                        push_uvs(&mut uvs, block_category);
                    }
                    if check_bottom(&chunk, x1, y1, z1) {
                        sides::bottom(x, y, z, &mut positions, &mut normals, &mut indices);
                        push_uvs(&mut uvs, block_category);
                    }
                }
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

const CHUNK_END: usize = CHUNK_SIZE - 1;

/* returns true if specific side should be rendered */
fn check_left(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if x > 0 {
        chunk.data[x - 1][y][z].is_transparent()
    } else {
        true
    }
}
fn check_right(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if x < CHUNK_END {
        chunk.data[x + 1][y][z].is_transparent()
    } else {
        true
    }
}
fn check_front(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if z < CHUNK_END {
        chunk.data[x][y][z + 1].is_transparent()
    } else {
        true
    }
}
fn check_back(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if z > 0 {
        chunk.data[x][y][z - 1].is_transparent()
    } else {
        true
    }
}
fn check_top(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if y < CHUNK_END {
        chunk.data[x][y + 1][z].is_transparent()
    } else {
        true
    }
}
fn check_bottom(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if y > 0 {
        chunk.data[x][y - 1][z].is_transparent()
    } else {
        true
    }
}