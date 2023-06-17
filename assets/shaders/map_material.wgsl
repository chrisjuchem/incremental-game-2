@group(1) @binding(0)
var<uniform> color1: vec4<f32>;
@group(1) @binding(1)
var<uniform> color2: vec4<f32>;

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
  var val = textureSample(noise_tex, noise_samp, input.uv)[0];
  var output = (color1 * val) + (color2 * (1.0 - val));
  return output;
}
