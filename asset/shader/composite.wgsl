const ACCUM_EPSILON: f32 = 0.00001;

@group(0) @binding(0)
var oit_accum: texture_2d<f32>;

@group(0) @binding(1)
var oit_reveal: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_ix: u32) -> VertexOutput {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );

    var out: VertexOutput;
    out.clip_position = vec4<f32>(positions[vertex_ix], 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = vec2<i32>(i32(position.x), i32(position.y));
    let accum = textureLoad(oit_accum, pixel, 0);
    let reveal = textureLoad(oit_reveal, pixel, 0).r;

    let alpha = clamp(1.0 - reveal, 0.0, 1.0);
    let color = accum.rgb / max(accum.a, ACCUM_EPSILON);
    return vec4<f32>(color * alpha, alpha);
}
