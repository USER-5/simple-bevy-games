#import bevy_pbr::forward_io::VertexOutput

struct CustomMaterial {
    // Unfortunately, all WASM-based shaders need to align with 16-bytes - so use vec4 here
    scale_factor: vec4<f32>,
    window: vec2<f32>,
    ball: vec2<f32>,
}

@group(1) @binding(0) var<uniform> mat: CustomMaterial;
@group(1) @binding(2) var colour_texture: texture_2d<f32>;
@group(1) @binding(3) var colour_sampler: sampler;

fn world_to_uv(world: vec2<f32>) -> vec2<f32> {
    return (world + mat.window / 2.) / mat.window;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Get the current texture coordinate in world space
    let world_coordinate: vec2<f32> = (mesh.position.xy - mat.window / 2.) * vec2<f32>(1., -1.);
    // Get a vector from the current coordinate to where the ball is
    let ball_delta = world_coordinate - (mat.ball * mat.scale_factor.x);
    let ball_direction = normalize(ball_delta);

    // Play with numbers to get a good level of distortion for this distance
    let distortion_magnitude = 1. + (500. / pow(length(ball_delta), 0.8));

    // Find the new "world space" sample for the texture
    let distortion_sample = ball_direction * distortion_magnitude + world_coordinate;

    // Darken areas near the ball
    let darkness_amount = saturate(pow(distortion_magnitude / 40., 3.0));

    // Resample the texture and darken
    return textureSample(colour_texture, colour_sampler, world_to_uv(distortion_sample)) * (1. - darkness_amount);
}

