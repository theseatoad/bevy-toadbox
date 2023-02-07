#import bevy_sprite::mesh2d_view_bindings
struct FadeMaterial {
    // Changes the speed of the fade.
    speed: f32,
    // Sin multiplier (changes the "wave feeling" of it.)
    mult: f32,
    min : f32,
    max : f32
};
@group(1) @binding(0)
var<uniform> material: FadeMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
    @builtin(position) coord : vec4<f32>
) -> @location(0) vec4<f32> {
    var t_1 = sin(globals.time * material.mult);
    t_1 = t_1 * material.speed;
    var texture = textureSample(base_color_texture, base_color_sampler,uv);
    var alpha = min(t_1,material.max);
    alpha = max(t_1, material.min);
    texture.a = alpha;
    return texture;
}