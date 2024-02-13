use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{ball::Ball, collisions::Collider, config::*, player::Player, CustomMaterial};

#[derive(Resource, Default, Clone, PartialEq, Eq, Debug)]
pub struct Scoreboard {
    pub left_player: u32,
    pub right_player: u32,
}

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut our_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    // Top
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                0.,
                (WINDOW_HEIGHT - WINDOW_PADDING) / 2.,
                0.,
            ))
            .with_scale((WINDOW_WIDTH, WINDOW_PADDING, 1.).into()),
            ..default()
        },
        Collider::Bounce,
    ));
    // Bottom
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                0.,
                (-WINDOW_HEIGHT + WINDOW_PADDING) / 2.,
                0.,
            ))
            .with_scale((WINDOW_WIDTH, WINDOW_PADDING, 1.).into()),
            ..default()
        },
        Collider::Bounce,
    ));
    // Left score zone
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::NONE)),
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2. - 50., 0., 0.))
                .with_scale((100., WINDOW_HEIGHT, 1.).into()),
            ..default()
        },
        Collider::Score(Player::Right),
    ));
    // Right score zone
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::NONE)),
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2. + 50., 0., 0.))
                .with_scale((100., WINDOW_HEIGHT, 1.).into()),
            ..default()
        },
        Collider::Score(Player::Left),
    ));

    let window = window.get_single().unwrap();
    // Background
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: -10.,
            })
            .with_scale(Vec3 {
                x: WINDOW_WIDTH,
                y: WINDOW_HEIGHT,
                z: 1.,
            }),
            material: our_materials.add(CustomMaterial {
                ball_location: Vec2 { x: 0.2, y: 0.5 },
                colour: asset_server.load("background.png"),
                window_size: Vec2 {
                    x: window.physical_width() as f32,
                    y: window.physical_height() as f32,
                },
                scale_factor: Vec4 {
                    x: window.scale_factor() as f32,
                    y: 0.,
                    z: 0.,
                    w: 0.,
                },
            }),
            ..default()
        },
        Background,
    ));
}

#[derive(Component)]
pub struct Background;

pub fn update_shader_effect(
    query: Query<&Transform, With<Ball>>,
    material_handle: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for transform in &query {
        let handle = material_handle.single();
        let mat = materials.get_mut(handle).unwrap();
        mat.ball_location = transform.translation.xy();
    }
}
