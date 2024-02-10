use crate::collisions::CollisionEvent;
use bevy::{audio::VolumeLevel, prelude::*};

#[derive(Resource)]
pub struct WallCollisionSound(Handle<AudioSource>);

pub fn load_collision_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_collision_sound = asset_server.load("paddle_hit.wav");
    commands.insert_resource(WallCollisionSound(ball_collision_sound));
}

pub fn play_wall_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<WallCollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    for _ in collision_events.read() {
        // This prevents events staying active on the next frame.
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN
                .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.2))),
        });
    }
}
