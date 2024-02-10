use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    collisions::CollisionEvent,
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    player::Player,
};

const MAX_SPEED: f32 = 1500.;
const INITIAL_BALL_VELOCITY: Vec2 = Vec2 { x: 400., y: 0. };
const INITIAL_BALL_LOCATION: Vec2 = Vec2 { x: 10., y: 0. };
const BALL_SIZE: f32 = 10.;
const SPEEDUP_RATIO: f32 = 1.1;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Event)]
pub struct RespawnBallEvent(pub Player);

pub fn spawn_ball(mut spawn_ball_events: EventWriter<RespawnBallEvent>) {
    // force a respawn of the ball
    spawn_ball_events.send(RespawnBallEvent(Player::Right));
}

pub fn respawn_ball(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_ball_events: EventReader<RespawnBallEvent>,
) {
    for owning_player in spawn_ball_events.read() {
        // despawn any existing ball before spawning
        if let Ok(ball) = ball_query.get_single() {
            commands.entity(ball).despawn();
        }

        // Spawn a new Ball
        let mut velocity = INITIAL_BALL_VELOCITY;
        if owning_player.0 == Player::Left {
            velocity = -velocity;
        }
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation((INITIAL_BALL_LOCATION, 0.).into())
                    .with_scale(Vec3::splat(BALL_SIZE * 2.)),
                ..default()
            },
            Ball,
            Velocity(velocity),
        ));
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

pub fn out_of_bounds(
    query: Query<&Transform, With<Ball>>,
    spawn_ball_events: EventWriter<RespawnBallEvent>,
) {
    let Ok(ball) = query.get_single() else {
        return;
    };

    if ball.translation.x.abs() > 5. * WINDOW_WIDTH || ball.translation.y.abs() > 5. * WINDOW_HEIGHT
    {
        spawn_ball(spawn_ball_events);
    }
}

pub fn speed_up(
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for mut ball_velocity in ball_query.iter_mut() {
        for _ in collisions.read() {
            ball_velocity.0 *= SPEEDUP_RATIO;
            ball_velocity.0 = ball_velocity.0.clamp_length_max(MAX_SPEED);
        }
    }
}
