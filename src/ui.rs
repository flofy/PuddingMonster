use bevy::prelude::*;

#[derive(Component)]
pub struct MovesText;

#[derive(Component)]
pub struct LevelText;

pub fn setup_ui(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Moves: 0",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        MovesText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Level: 1",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        LevelText,
    ));
}

pub fn update_ui_system(
    mut queries: QuerySet<(
        Query<&mut Text, With<MovesText>>,
        Query<&mut Text, With<LevelText>>,
    )>,
    game_state: Res<GameState>,
    level: Res<CurrentLevel>,
) {
    let Ok(q) = queries.get_single() else { return };
    let Ok(mut moves_text) = q.0.get_single_mut() else { return };
    let Ok(mut level_text) = q.1.get_single_mut() else { return };

    moves_text.sections[0].value = format!("Moves: {}", game_state.moves_remaining);
    level_text.sections[0].value = format!("Level: {}", level.index + 1);
}

// Import GameState and CurrentLevel from main module
use crate::{GameState, CurrentLevel};
