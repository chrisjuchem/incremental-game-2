@group(1) @binding(0)
var color_tex: texture_2d<f32>;
@group(1) @binding(1)
var color_samp: sampler;

@group(1) @binding(2)
var noise_tex: texture_2d<f32>;
@group(1) @binding(3)
var noise_samp: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> { //location 0 == color
    var noise = textureSample(noise_tex, noise_samp, input.uv);
    // noise values in range (0.3, 0.7)

    var temp = noise.r;
    var water = noise.b;

//    temp = temp + abs(input.uv.y - 0.5) * 0.6; // temp adjust by latitude
    temp = (temp - 0.3) * 2.5;

    // expand water to full range
    // (0.3, 0.7) --(-0.3)--> (0.0, 0.4) --(* 2.5)--> (0.0, 1.0)
    water = (water - 0.3) * 2.5;

    var color = textureSample(color_tex, color_samp, vec2f( water, temp));

    return color;
}
