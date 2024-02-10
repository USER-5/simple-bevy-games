use bevy::prelude::*;
use bevy::{
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    render::color::Color,
    text::{Text, TextAlignment, TextStyle},
    ui::{node_bundles::TextBundle, JustifySelf, Style},
};

#[derive(Component)]
pub struct ScoreBoardUi;

use crate::{collisions::ScoreEvent, level::Scoreboard, player::Player};

pub fn add_score(mut scoreboard: ResMut<Scoreboard>, mut score_events: EventReader<ScoreEvent>) {
    if scoreboard.is_changed() {
        return;
    }
    for score_event in score_events.read() {
        match score_event.0 {
            Player::Left => scoreboard.left_player += 1,
            Player::Right => scoreboard.right_player += 1,
        }
    }
}

pub fn update_scoreboard(
    score: Res<Scoreboard>,
    mut score_ui: Query<&mut Text, With<ScoreBoardUi>>,
) {
    for mut scoreboard in score_ui.iter_mut() {
        if let Some(text_section) = scoreboard.sections.get_mut(0) {
            text_section.value = format!("{}:{}", score.left_player, score.right_player);
        }
    }
}

pub fn create_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Test",
            TextStyle {
                font_size: 50.,
                color: Color::GRAY,
                ..default()
            },
        )
        .with_style(Style {
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        })
        .with_text_alignment(TextAlignment::Center),
        ScoreBoardUi,
    ));
}
