use gdnative::prelude::*;
use super::sides::*;

use protocol::chunk::CHUNK_SIZE;
use protocol::chunk::Block;
use noise::{NoiseFn, OpenSimplex};

#[allow(clippy::needless_range_loop)]
pub fn fix(
    data: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    sides: [Option<[[Block; CHUNK_SIZE]; CHUNK_SIZE]>; 6],
    verts: &mut Vector3Array,
    colors: &mut ColorArray,
    normals: &mut Vector3Array,
    indices: &mut PoolArray<i32>,
) {
    let noise = OpenSimplex::new();
    if let Some(left) = sides[0] {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let offset: f32 = noise.get([0.0, y as f64, z as f64]) as f32 / 4.0;
                let own_block = data[0][y][z];
                let side_block = left[z][y];

                let c = side_block.to_color();
                let side_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                let c = own_block.to_color();
                let own_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = RIGHT.clone().apply_vertex_position(Vector3 {
                        x: -1.0,
                        y: y as f32,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, side_color);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = LEFT.clone().apply_vertex_position(Vector3 {
                        x: 0.0,
                        y: y as f32,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, own_color);
                }
            }
        }
    }
    if let Some(right) = sides[1] {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let offset: f32 = noise.get([CHUNK_SIZE as f64, y as f64, z as f64]) as f32 / 4.0;
                let own_block = data[CHUNK_SIZE - 1][y][z];
                let side_block = right[z][y];
                let c = side_block.to_color();
                let side_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                let c = own_block.to_color();
                let own_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = LEFT.clone().apply_vertex_position(Vector3 {
                        x: CHUNK_SIZE as f32,
                        y: y as f32,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, side_color);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = RIGHT.clone().apply_vertex_position(Vector3 {
                        x: (CHUNK_SIZE - 1) as f32,
                        y: y as f32,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, own_color);
                }
            }
        }
    }
    // DOES NOT WORK
    if let Some(front) = sides[2] {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let offset: f32 = noise.get([x as f64, y as f64, 0.0]) as f32 / 4.0;
                let own_block = data[x][y][CHUNK_SIZE - 1];
                let side_block = front[x][y];
                let c = side_block.to_color();
                let side_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                let c = own_block.to_color();
                let own_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = BACK.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: y as f32,
                        z: CHUNK_SIZE as f32,
                    });
                    side.push(verts, normals, colors, indices, side_color);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = FRONT.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: y as f32,
                        z: (CHUNK_SIZE - 1) as f32,
                    });
                    side.push(verts, normals, colors, indices, own_color);
                }
            }
        }
    }
    if let Some(back) = sides[3] {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let offset: f32 = noise.get([x as f64, y as f64, CHUNK_SIZE as f64]) as f32 / 4.0;
                let own_block = data[x][y][0];
                let side_block = back[x][y];
                let c = side_block.to_color();
                let side_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                let c = own_block.to_color();
                let own_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = FRONT.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: y as f32,
                        z: -1.0,
                    });
                    side.push(verts, normals, colors, indices, side_color);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = BACK.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: y as f32,
                        z: 0.0,
                    });
                    side.push(verts, normals, colors, indices, own_color);
                }
            }
        }
    }
    if let Some(top) = sides[4] {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let offset: f32 = noise.get([x as f64, CHUNK_SIZE as f64, z as f64]) as f32 / 4.0;
                let own_block = data[x][CHUNK_SIZE - 1][z];
                let side_block = top[x][z];
                let c = side_block.to_color();
                let side_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                let c = own_block.to_color();
                let own_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = BOTTOM.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: CHUNK_SIZE as f32,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, side_color);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = TOP.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: (CHUNK_SIZE - 1) as f32,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, own_color);
                }
            }
        }
    }
    if let Some(bottom) = sides[5] {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let offset: f32 = noise.get([x as f64, 0.0, z as f64]) as f32 / 4.0;
                let own_block = data[x][0][z];
                let side_block = bottom[x][z];
                let c = side_block.to_color();
                let side_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                let c = own_block.to_color();
                let own_color = [
                    c[0] as f32 / 255.0 + offset,
                    c[1] as f32 / 255.0 + offset,
                    c[2] as f32 / 255.0 + offset,
                    1.0
                ];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = TOP.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: -1.0,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, side_color);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = BOTTOM.clone().apply_vertex_position(Vector3 {
                        x: x as f32,
                        y: 0.0,
                        z: z as f32,
                    });
                    side.push(verts, normals, colors, indices, own_color);
                }
            }
        }
    }
}