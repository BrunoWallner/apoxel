// TAKES 120 MILLISECONDS, HUGE PERFORMANCE / LATENCY PENALTY;
// but at least it runs not on main thread;


use gdnative::prelude::*;

use protocol::chunk::CHUNK_SIZE;
use protocol::Coord;

use gdnative::api::{ArrayMesh, CollisionShape, Mesh, MeshInstance, StaticBody, SpatialMaterial};

pub fn gen(
    data: (
        Coord,
        Vector3Array,
        Vector3Array,
        ColorArray,
        PoolArray<i32>,
    ),
) -> Option<Ref<Spatial>> {
    let coord = data.0;
    let verts = data.1;
    let normals = data.2;
    let colors = data.3;
    let indices = data.4;

    if !verts.is_empty() {
        //return (coord, verts, uvs, normals, indices);
        let arr = VariantArray::new();
        arr.resize(Mesh::ARRAY_MAX as i32);

        arr.set(Mesh::ARRAY_VERTEX as i32, verts);
        // arr.set(Mesh::ARRAY_TEX_UV as i32, uvs);
        arr.set(Mesh::ARRAY_NORMAL as i32, normals);
        arr.set(Mesh::ARRAY_COLOR as i32, colors);
        arr.set(Mesh::ARRAY_INDEX as i32, indices);

        let array_mesh = ArrayMesh::new();
        array_mesh.add_surface_from_arrays(
            Mesh::PRIMITIVE_TRIANGLES,
            arr.into_shared(),
            VariantArray::new().into_shared(),
            2194432,
        );
        let mesh_collision_shape = array_mesh.create_trimesh_shape();

        let collision_shape = CollisionShape::new();
        if let Some(shape) = mesh_collision_shape {
            collision_shape.set_shape(shape);
        }

        let static_body = StaticBody::new();
        static_body.add_child(collision_shape, false);

        // for vertex colors
        let spatial_material = SpatialMaterial::new();
        spatial_material.set_flag(SpatialMaterial::FLAG_ALBEDO_FROM_VERTEX_COLOR, true);

        let mesh_instance = MeshInstance::new();
        mesh_instance.set_mesh(array_mesh);
        mesh_instance.set_surface_material(0, spatial_material);
        mesh_instance.add_child(static_body, false);

        let spatial = Spatial::new();
        spatial.translate(Vector3 {
            x: coord[0] as f32 * CHUNK_SIZE as f32,
            y: coord[1] as f32 * CHUNK_SIZE as f32,
            z: coord[2] as f32 * CHUNK_SIZE as f32,
        });
        spatial.add_child(mesh_instance, false);

        Some(spatial.into_shared())
    } else {
        None
    }
}
