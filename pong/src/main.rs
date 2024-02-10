use ball::{apply_velocity, respawn_ball, spawn_ball, speed_up};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
    window::WindowResolution,
};
use collisions::check_for_collisions;
use level::{spawn_world, update_shader_effect, Scoreboard};
use player::{move_player, spawn_player_one, spawn_player_two};
use scoreboard::{add_score, create_scoreboard, update_scoreboard};
use sounds::{load_collision_sounds, play_wall_collision_sound};
pub mod ball;
pub mod collisions;
pub mod config;
pub mod level;
pub mod player;
pub mod scoreboard;
pub mod sounds;

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub window_size: Vec2,
    #[uniform(0)]
    pub ball_location: Vec2,
    #[texture(2)]
    #[sampler(3)]
    pub colour: Handle<Image>,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Pong".to_string(),
                resolution: WindowResolution::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .insert_resource(Scoreboard {
            left_player: 0,
            right_player: 0,
        })
        .add_event::<collisions::CollisionEvent>()
        .add_event::<collisions::ScoreEvent>()
        .add_event::<ball::RespawnBallEvent>()
        .add_systems(
            Startup,
            (
                setup,
                spawn_player_one,
                spawn_player_two,
                spawn_ball,
                spawn_world,
                create_scoreboard,
                load_collision_sounds,
            ),
        )
        .add_systems(
            Update,
            (
                move_player,
                add_score,
                respawn_ball,
                update_scoreboard,
                speed_up,
                play_wall_collision_sound,
                update_shader_effect,
            ),
        )
        .add_systems(FixedUpdate, (check_for_collisions, apply_velocity).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::hex("#000").unwrap()),
        },
        ..default()
    });
}
