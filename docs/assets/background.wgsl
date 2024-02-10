#import bevy_pbr::forward_io::VertexOutput

struct CustomMaterial {
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
    let world_coordinate: vec2<f32> = (mesh.position.xy - mat.window / 2.) * vec2<f32>(1., -1.);
    let ball_delta = world_coordinate - mat.ball;
    let ball_direction = normalize(ball_delta);

    let distortion_magnitude = 1. + (500. / pow(length(ball_delta), 0.8));

    let distortion_sample = ball_direction * distortion_magnitude + world_coordinate;

    let darkness_amount = saturate(pow(distortion_magnitude / 40., 3.0));

    return textureSample(colour_texture, colour_sampler, world_to_uv(distortion_sample)) * (1. - darkness_amount);
}

