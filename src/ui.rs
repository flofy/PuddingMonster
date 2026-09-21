use bevy::prelude::*;
use crate::{GameState, CurrentLevel, CurrentWorld, WORLDS, WorldTheme};

#[derive(Component)]
pub struct MovesText;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct WorldText;

#[derive(Component)]
pub struct StarsText;

pub fn setup_ui(commands: &mut Commands) {
    // Moves remaining
    commands.spawn((
        TextBundle::from_section(
            "Moves: 0",
            TextStyle {
                font_size: 20.0,
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

    // Current level
    commands.spawn((
        TextBundle::from_section(
            "Level: 1-1",
            TextStyle {
                font_size: 20.0,
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

    // World name
    commands.spawn((
        TextBundle::from_section(
            "World: Forest",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        }),
        WorldText,
    ));

    // Stars collected
    commands.spawn((
        TextBundle::from_section(
            "Stars: 0/0",
            TextStyle {
                font_size: 20.0,
                color: Color::YELLOW,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            right: Val::Px(10.0),
            ..default()
        }),
        StarsText,
    ));
}

pub fn update_ui_system(
    mut moves_query: Query<&mut Text, With<MovesText>>,
    mut level_query: Query<&mut Text, With<LevelText>>,
    mut world_query: Query<&mut Text, With<WorldText>>,
    mut stars_query: Query<&mut Text, With<StarsText>>,
    game_state: Res<GameState>,
    level: Res<CurrentLevel>,
    world: Res<CurrentWorld>,
    collected_stars: Res<CollectedStars>,
) {
    if let Ok(mut moves_text) = moves_query.get_single_mut() {
        moves_text.sections[0].value = format!("Moves: {}", game_state.moves_remaining);
    }

    if let Ok(mut level_text) = level_query.get_single_mut() {
        level_text.sections[0].value = format!("Level: {}-{}", world.index + 1, level.index + 1);
    }

    if let Ok(mut world_text) = world_query.get_single_mut() {
        world_text.sections[0].value = format!("World: {}", WORLDS[world.index].name);
    }

    if let Ok(mut stars_text) = stars_query.get_single_mut() {
        let collected = collected_stars.stars.iter().filter(|&&collected| collected).count();
        stars_text.sections[0].value = format!("Stars: {}/{}", collected, level.data.stars.len());
    }
}

// Import types
use crate::pudding::Pudding;
use crate::CollectedStars;