mod sides;
mod borders;

use gdnative::prelude::*;
use noise::{NoiseFn, OpenSimplex};

use sides::*;

use protocol::chunk::Block;
use protocol::chunk::Chunk;
use protocol::chunk::CHUNK_SIZE;
use protocol::Coord;

pub fn gen(
    chunk: Chunk,
    sides: [Option<[[Block; CHUNK_SIZE]; CHUNK_SIZE]>; 6],
) -> (
    Coord,
    Vector3Array,
    Vector3Array,
    ColorArray,
    PoolArray<i32>,
) {
    let coord = chunk.coord;

    let data = chunk.data;
    let mut verts = Vector3Array::new();
    let mut normals = Vector3Array::new();
    let mut colors = PoolArray::new();
    let mut indices: PoolArray<i32> = PoolArray::new();

    let noise = OpenSimplex::new();
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if data[x][y][z].to_category().0 != 0 {
                    let c = data[x][y][z].to_color();
                    let offset: f32 = noise.get([x as f64, y as f64, z as f64]) as f32 / 4.0;
                    let color = [
                        c[0] as f32 / 255.0 + offset,
                        c[1] as f32 / 255.0 + offset,
                        c[2] as f32 / 255.0 + offset,
                        data[x][y][z].transparency().unwrap_or(1.0),
                    ];
                    if check_left(x, y, z, &data) {
                        let side = LEFT.clone().apply_vertex_position(Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        });
                        side.push(&mut verts, &mut normals, &mut colors, &mut indices, color);
                    }
                    if check_right(x, y, z, &data) {
                        let side = RIGHT.clone().apply_vertex_position(Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        });
                        side.push(&mut verts, &mut normals, &mut colors, &mut indices, color);
                    }
                    if check_front(x, y, z, &data) {
                        let side = FRONT.clone().apply_vertex_position(Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        });
                        side.push(&mut verts, &mut normals, &mut colors, &mut indices, color);
                    }
                    if check_back(x, y, z, &data) {
                        let side = BACK.clone().apply_vertex_position(Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        });
                        side.push(&mut verts, &mut normals, &mut colors, &mut indices, color);
                    }
                    if check_top(x, y, z, &data) {
                        let side = TOP.clone().apply_vertex_position(Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        });
                        side.push(&mut verts, &mut normals, &mut colors, &mut indices, color);
                    }
                    if check_bottom(x, y, z, &data) {
                        let side = BOTTOM.clone().apply_vertex_position(Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        });
                        side.push(&mut verts, &mut normals, &mut colors, &mut indices, color);
                    }
                }
            }
        }
    }

    borders::fix(
        data,
        sides,
        &mut verts,
        &mut colors,
        &mut normals,
        &mut indices,
    );

    (coord, verts, normals, colors, indices)
}