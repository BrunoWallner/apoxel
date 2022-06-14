#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import "shaders/noise.wgsl"

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] light: f32;
    [[location(3)]] normal: vec3<f32>;
};

struct CustomMaterial {
    color: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] light: f32;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    
    out.color = vertex.color;
    let color_offset: f32 = noise(vertex.normal) / 50.0;
    out.color[0] = out.color[0] + color_offset;
    out.color[1] = out.color[1] + color_offset;
    out.color[2] = out.color[2] + color_offset;

    out.clip_position = view.view_proj * world_position;
    out.light = vertex.light;
    return out;
}

struct FragmentInput {
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] light: f32;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
    return input.color * pow(input.light, 0.75);
}