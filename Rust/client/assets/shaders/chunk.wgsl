#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import "shaders/noise.wgsl"

let RENDER_DISTANCE: f32 = 300.0;

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
    [[location(2)]] fog: f32;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position: vec4<f32> = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    
    out.color = vertex.color;
    let noise_pos = vec3<f32>(
        vertex.normal[0] * 0.5,
        vertex.normal[1] * 0.5,
        vertex.normal[2] * 0.5,
    );
    let color_offset: f32 = noise(noise_pos) / 20.0;
    out.color[0] = out.color[0] + color_offset;
    out.color[1] = out.color[1] + color_offset;
    out.color[2] = out.color[2] + color_offset;

    out.clip_position = view.view_proj * world_position;
    out.light = vertex.light;

    let player_position: vec4<f32> = view.view[3];
    let distance: f32 = distance(world_position, player_position);
    out.fog = clamp((distance / RENDER_DISTANCE - 0.5), 0.0, 2.0);

    return out;
}

struct FragmentInput {
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] light: f32;
    [[location(2)]] fog: f32;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
    return input.color * input.light - vec4<f32>(0.75, 1.0, 0.5, 1.0) * input.fog;
}