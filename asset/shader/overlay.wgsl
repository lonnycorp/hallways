// Group 0: Textures
@group(0) @binding(0)
var diffuse_sampler: sampler;
@group(0) @binding(1)
var diffuse: texture_2d_array<f32>;

struct PushConstants {
    resolution: vec2<f32>,
}
var<push_constant> pc: PushConstants;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) texture_ix: u32,
    @location(3) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(flat) texture_ix: u32,
    @location(2) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let ndc = in.position / pc.resolution * 2.0 - vec2<f32>(1.0, 1.0);
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    let tex_size = vec2<f32>(textureDimensions(diffuse));
    out.uv = in.uv / tex_size;
    out.texture_ix = in.texture_ix;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(diffuse, diffuse_sampler, in.uv, i32(in.texture_ix));
    return sampled * in.color;
}
