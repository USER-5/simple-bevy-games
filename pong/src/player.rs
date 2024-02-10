use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::collisions::Collider;
use crate::config;

const PLAYER_SPEED: f32 = 600.;
const PADDLE_SIZE: Vec2 = Vec2 { x: 20., y: 100. };
const PLAYER_ANGLE_STRENGTH: f32 = 4.;

const PLAYER_X_MAGNITUDE: f32 =
    (config::WINDOW_WIDTH / 2.) - config::WINDOW_PADDING - (PADDLE_SIZE.x / 2.);

#[derive(Component, PartialEq, Debug, Clone)]
pub enum Player {
    Left,
    Right,
}

pub fn spawn_player_one(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-PLAYER_X_MAGNITUDE, 0., 0.))
                .with_scale((PADDLE_SIZE, 1.).into()),
            ..default()
        },
        Player::Left,
        Collider::Angle(PLAYER_ANGLE_STRENGTH),
    ));
}

pub fn spawn_player_two(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(PLAYER_X_MAGNITUDE, 0., 0.))
                .with_scale((PADDLE_SIZE, 1.).into()),
            ..default()
        },
        Player::Right,
        Collider::Angle(PLAYER_ANGLE_STRENGTH),
    ));
}

const PLAYER_Y_MAX_MAGNITUDE: f32 =
    (config::WINDOW_HEIGHT - PADDLE_SIZE.y) / 2. - config::WINDOW_PADDING;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut player_transform, player_type) in query.iter_mut() {
        // Query the right keycodes, depending on the player
        let (upkey, downkey) = if player_type == &Player::Right {
            (KeyCode::Up, KeyCode::Down)
        } else {
            (KeyCode::W, KeyCode::S)
        };
        let mut direction = 0.;
        if keyboard_input.pressed(upkey) {
            direction += 1.;
        }
        if keyboard_input.pressed(downkey) {
            direction -= 1.;
        }
        let new_position =
            player_transform.translation.y + direction * PLAYER_SPEED * time.delta_seconds();
        player_transform.translation.y =
            new_position.clamp(-PLAYER_Y_MAX_MAGNITUDE, PLAYER_Y_MAX_MAGNITUDE);
    }
}
