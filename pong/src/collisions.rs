use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    ball::{Ball, RespawnBallEvent, Velocity},
    player::Player,
};

#[derive(Event)]
pub enum CollisionEvent {
    Wall,
    Player,
}

#[derive(Event)]
pub struct ScoreEvent(pub Player);

#[derive(Component)]
pub enum Collider {
    Bounce,
    Angle(f32),
    Score(Player),
}

pub fn check_for_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, &Collider)>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut score_events: EventWriter<ScoreEvent>,
    mut respawn_ball_events: EventWriter<RespawnBallEvent>,
) {
    let Ok((mut ball_velocity, ball_transform)) = ball_query.get_single_mut() else {
        return;
    };
    let ball_size = ball_transform.scale.truncate();

    for (other_transform, other_collider) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            other_transform.translation,
            other_transform.scale.truncate(),
        );
        let Some(collision) = collision  else {
            continue;
        };
        match other_collider {
            Collider::Score(player) => {
                score_events.send(ScoreEvent(player.clone()));
                respawn_ball_events.send(RespawnBallEvent(player.clone()));
                return;
            }
            Collider::Angle(angle_strength) => {
                // angle the bounce, apply "bounce" as well
                let original_magnitude = ball_velocity.0.length();
                let y_difference = other_transform.translation.y - ball_transform.translation.y;
                ball_velocity.0.y -= y_difference * angle_strength;
                ball_velocity.0 = ball_velocity.0.clamp_length_max(original_magnitude);
                collision_events.send(CollisionEvent::Player)
            }
            // apply outside of the match scope
            Collider::Bounce => collision_events.send(CollisionEvent::Wall),
        }

        let mut reflect_x = false;
        let mut reflect_y = false;

        match collision {
            Collision::Left => reflect_x = ball_velocity.0.x > 0.,
            Collision::Right => reflect_x = ball_velocity.0.x < 0.,
            Collision::Top => reflect_y = ball_velocity.0.y < 0.,
            Collision::Bottom => reflect_y = ball_velocity.0.y > 0.,
            Collision::Inside => {}
        }

        if reflect_x {
            ball_velocity.0.x = -ball_velocity.0.x;
        }

        if reflect_y {
            ball_velocity.0.y = -ball_velocity.0.y;
        }
    }
}
