// Vertex shader
struct Uniforms {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct InstanceIn {
    @location(5) position: vec3<i32>,
    @location(6) block_type: u32
}

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
}

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
}

@vertex
fn vs_main(in: VertexIn, instance: InstanceIn) -> VertexOut {

    var out: VertexOut;

    let x = in.position.x + f32(instance.position.x * 2);
    let y = in.position.y + f32(instance.position.y * 2);
    let z = in.position.z + f32(instance.position.z * 2);

    out.position = uniforms.view_proj * vec4<f32>(x, y, z, 1.0);
    out.tex_coords = in.tex_coords;

    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}