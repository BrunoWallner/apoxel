mod sides;
mod light;

use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::mesh::Indices;

use protocol::chunk::{CHUNK_SIZE, Chunk};
use super::chunk_material::ATTRIBUTE_COLOR;
use super::chunk_material::ATTRIBUTE_LIGHT;

// ca. 150 - 1200 µs
pub fn generate(
    chunk: &Chunk,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let v_length = 8 * CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(v_length);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(v_length);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(v_length);
    let mut lights: Vec<f32> = Vec::with_capacity(v_length);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(v_length);

    let mut indices: Vec<u32> = Vec::with_capacity(v_length);

    for x1 in 0..CHUNK_SIZE {
        for y1 in 0..CHUNK_SIZE {
            for z1 in 0..CHUNK_SIZE {

                let x: f32 = x1 as f32;
                let y: f32 = y1 as f32;
                let z: f32= z1 as f32;

                let block_category = chunk.data.get(x1, y1, z1).to_category();
                
                // air
                if block_category.0 != 0 {
                    let mut sides: u8 = 0;
                    if check_left(&chunk, x1, y1, z1) {
                        sides::left(x, y, z, &mut positions, &mut normals, &mut indices);
                        light::left(x1, y1, z1, &mut lights, &chunk.data);
                        sides += 1;
                    }
                    if check_right(&chunk, x1, y1, z1) {
                        sides::right(x, y, z, &mut positions, &mut normals, &mut indices);
                        light::right(x1, y1, z1, &mut lights, &chunk.data);
                        sides += 1;
                    }
                    if check_back(&chunk, x1, y1, z1) {
                        sides::back(x, y, z, &mut positions, &mut normals, &mut indices);
                        light::back(x1, y1, z1, &mut lights, &chunk.data);
                        sides += 1;
                    }
                    if check_front(&chunk, x1, y1, z1) {
                        sides::front(x, y, z, &mut positions, &mut normals, &mut indices);
                        light::front(x1, y1, z1, &mut lights, &chunk.data);
                        sides += 1;
                    }
                    if check_top(&chunk, x1, y1, z1) {
                        sides::top(x, y, z, &mut positions, &mut normals, &mut indices);
                        light::top(x1, y1, z1, &mut lights, &chunk.data);
                        sides += 1;
                    }
                    if check_bottom(&chunk, x1, y1, z1) {
                        sides::bottom(x, y, z, &mut positions, &mut normals, &mut indices);
                        light::bottom(x1, y1, z1, &mut lights, &chunk.data);
                        sides += 1;
                    }

                    let c = chunk.data.get(x1, y1, z1).to_color();
                    // let offset = noise.get([
                    //     (chunk.coord[0] as f64 * CHUNK_SIZE as f64 + x as f64) * 12.5,
                    //     (chunk.coord[1] as f64 * CHUNK_SIZE as f64 + y as f64) * 12.5,
                    //     (chunk.coord[2] as f64 * CHUNK_SIZE as f64 + z as f64) * 12.5,
                    // ]) as f32 / 5.0;
                    let offset = 0.0;
                    
                    for _ in 0..sides * 4 {
                        colors.push([
                            c[0] as f32 / 255.0 + offset,
                            c[1] as f32 / 255.0 + offset,
                            c[2] as f32 / 255.0 + offset,
                            0.5
                        ]);
                        uvs.push([0.0, 0.0]);
                    }
                }
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(ATTRIBUTE_LIGHT, lights);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

const CHUNK_END: usize = CHUNK_SIZE - 1;

/* returns true if specific side should be rendered */
fn check_left(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if x > 0 {
        chunk.data.get(x - 1, y, z).transparency().is_some()
    } else {
        true
    }
}
fn check_right(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if x < CHUNK_END {
        chunk.data.get(x + 1, y, z).transparency().is_some()
    } else {
        true
    }
}
fn check_front(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if z < CHUNK_END {
        chunk.data.get(x, y, z + 1).transparency().is_some()
    } else {
        true
    }
}
fn check_back(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if z > 0 {
        chunk.data.get(x, y, z - 1).transparency().is_some()
    } else {
        true
    }
}
fn check_top(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if y < CHUNK_END {
        chunk.data.get(x, y + 1, z).transparency().is_some()
    } else {
        true
    }
}
fn check_bottom(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    if y > 0 {
        chunk.data.get(x, y - 1, z).transparency().is_some()
    } else {
        true
    }
}