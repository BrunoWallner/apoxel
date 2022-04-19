use gdnative::prelude::*;

use super::sides::*;

use protocol::chunk::CHUNK_SIZE;
use protocol::chunk::Chunk;
use protocol::chunk::Block;
use protocol::Coord;

pub fn gen(chunk: Chunk, sides: [Option<[[Block; CHUNK_SIZE]; CHUNK_SIZE]>; 6]) -> (Coord, Vector3Array, Vector2Array, Vector3Array, PoolArray<i32>) {
    let coord = chunk.coord;

    let data = chunk.data;
    let mut verts = Vector3Array::new();
    let mut uvs = Vector2Array::new();
    let mut normals = Vector3Array::new();
    let mut indices: PoolArray<i32> = PoolArray::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if data[x][y][z].to_category().0 != 0 {
                    if check_left(x, y, z, &data) {
                        let side = LEFT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(&mut verts, &mut uvs, &mut normals, &mut indices, &side);
                    }
                    if check_right(x, y, z, &data) {
                        let side = RIGHT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(&mut verts, &mut uvs, &mut normals, &mut indices, &side);
                    }
                    if check_front(x, y, z, &data) {
                        let side = FRONT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(&mut verts, &mut uvs, &mut normals, &mut indices, &side);
                    }
                    if check_back(x, y, z, &data) {
                        let side = BACK.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(&mut verts, &mut uvs, &mut normals, &mut indices, &side);
                    }
                    if check_top(x, y, z, &data) {
                        let side = TOP.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(&mut verts, &mut uvs, &mut normals, &mut indices, &side);
                    }
                    if check_bottom(x, y, z, &data) {
                        let side = BOTTOM.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: z as f32});
                        push_side(&mut verts, &mut uvs, &mut normals, &mut indices, &side);
                    }
                }
            }
        }
    }

    fix_borders(
        data,
        sides,
        &mut verts,
        &mut uvs,
        &mut normals,
        &mut indices,
    );

    (coord, verts, uvs, normals, indices)
}

#[allow(clippy::needless_range_loop)]
fn fix_borders(
    data: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    sides: [Option<[[Block; CHUNK_SIZE]; CHUNK_SIZE]>; 6],
    verts: &mut Vector3Array, 
    uvs: &mut Vector2Array, 
    normals: &mut Vector3Array,
    indices: &mut PoolArray<i32>,
) {
    if let Some(left) = sides[0] {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let own_block = data[0][y][z];
                let side_block = left[z][y];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = RIGHT.clone().apply_vertex_position(Vector3{x: -1.0, y: y as f32, z: z as f32});
                    push_side(verts, uvs, normals, indices, &side);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = LEFT.clone().apply_vertex_position(Vector3{x: 0.0, y: y as f32, z: z as f32});
                    push_side(verts, uvs, normals, indices, &side);
                }
            }
        }
    }
    if let Some(right) = sides[1] {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let own_block = data[CHUNK_SIZE - 1][y][z];
                let side_block = right[z][y];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = LEFT.clone().apply_vertex_position(Vector3{x: CHUNK_SIZE as f32, y: y as f32, z: z as f32});
                    push_side(verts, uvs, normals, indices, &side);
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = RIGHT.clone().apply_vertex_position(Vector3{x: (CHUNK_SIZE - 1) as f32, y: y as f32, z: z as f32});
                    push_side(verts, uvs, normals, indices, &side);
                }
            }
        }
    }
    // DOES NOT WORK
    if let Some(front) = sides[2] {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let own_block = data[x][y][CHUNK_SIZE - 1];
                let side_block = front[x][y];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = BACK.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: CHUNK_SIZE as f32});
                    push_side(verts, uvs, normals, indices, &side); 
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = FRONT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: (CHUNK_SIZE - 1) as f32});
                    push_side(verts, uvs, normals, indices, &side);
                }
            }
        }
    }
    if let Some(back) = sides[3] {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let own_block = data[x][y][0];
                let side_block = back[x][y];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = FRONT.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: -1.0});
                    push_side(verts, uvs, normals, indices, &side); 
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = BACK.clone().apply_vertex_position(Vector3{x: x as f32, y: y as f32, z: 0.0}); 
                    push_side(verts, uvs, normals, indices, &side); 
                }
            }
        }  
    }
    if let Some(top) = sides[4] {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let own_block = data[x][CHUNK_SIZE - 1][z];
                let side_block = top[x][z];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = BOTTOM.clone().apply_vertex_position(Vector3{x: x as f32, y: CHUNK_SIZE as f32, z: z as f32});
                    push_side(verts, uvs, normals, indices, &side); 
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = TOP.clone().apply_vertex_position(Vector3{x: x as f32, y: (CHUNK_SIZE - 1) as f32, z: z as f32}); 
                    push_side(verts, uvs, normals, indices, &side); 
                }
            }
        }  
    }
    if let Some(bottom) = sides[5] {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let own_block = data[x][0][z];
                let side_block = bottom[x][z];
                if own_block.to_category().0 == 0 && side_block.to_category().0 != 0 {
                    let side = TOP.clone().apply_vertex_position(Vector3{x: x as f32, y: -1.0, z: z as f32});
                    push_side(verts, uvs, normals, indices, &side); 
                }
                if own_block.to_category().0 != 0 && side_block.to_category().0 == 0 {
                    let side = BOTTOM.clone().apply_vertex_position(Vector3{x: x as f32, y: 0.0, z: z as f32}); 
                    push_side(verts, uvs, normals, indices, &side); 
                }
            }
        }  
    }
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