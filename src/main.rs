use bevy::prelude::*;
use bevy::window::WindowMode;

mod pudding;
mod levels;
mod world;
mod ui;
mod star;

use pudding::*;
use levels::*;
use world::*;
use ui::*;
use star::*;

// Import types for events
use pudding::PuddingDragEvent;
use star::StarCollectedEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pudding Monsters".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_event::<PuddingDragEvent>()
        .add_event::<PuddingMergeEvent>()
        .add_event::<GameWinEvent>()
        .add_event::<StarCollectedEvent>()
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .init_resource::<CurrentLevel>()
            .init_resource::<CurrentWorld>()
            .init_resource::<UnlockedWorlds>()
            .init_resource::<CollectedStars>()
            .add_systems(Startup, setup_game)
            .add_systems(Update, (
                handle_pudding_drag,
                pudding::handle_telepathic_movement,
                handle_pudding_movement,
                pudding::handle_slime_trail,
                check_pudding_collisions,
                check_star_collection,
                check_win_condition,
                handle_level_transition,
            ).chain())
            .add_systems(Update, ui::update_ui_system);
    }
}

#[derive(Resource)]
struct GameState {
    moves_remaining: u32,
    is_dragging: bool,
    drag_start: Option<Vec2>,
    selected_pudding: Option<Entity>,
    stars_collected: usize,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            moves_remaining: 0,
            is_dragging: false,
            drag_start: None,
            selected_pudding: None,
            stars_collected: 0,
        }
    }
}

#[derive(Resource)]
struct CurrentLevel {
    pub index: usize,
    pub data: LevelData,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            index: 0,
            data: WORLDS[0].levels[0].clone(),
        }
    }
}

#[derive(Resource)]
struct CollectedStars {
    pub stars: Vec<bool>,
}

impl Default for CollectedStars {
    fn default() -> Self {
        Self { stars: Vec::new() }
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level: ResMut<CurrentLevel>,
    mut world: ResMut<CurrentWorld>,
    mut game_state: ResMut<GameState>,
) {
    // Setup camera
    commands.spawn(Camera2dBundle::default());

    // Load first level from first world
    *world = CurrentWorld { index: 0 };
    *level = CurrentLevel {
        index: 0,
        data: WORLDS[0].levels[0].clone(),
    };
    game_state.moves_remaining = level.data.max_moves;
    game_state.stars_collected = 0;

    // Spawn puddings for the first level
    spawn_level(&mut commands, &mut meshes, &mut materials, &level.data);

    // Spawn stars for the first level
    star::setup_stars(&mut commands, &level.data.stars);

    // Setup UI
    ui::setup_ui(&mut commands);
}

fn spawn_level(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    level_data: &LevelData,
) {
    // Spawn board background based on world theme
    let bg_color = match WORLDS[0].theme { // Will be fixed when we have access to current world
        WorldTheme::Forest => Color::DARK_GREEN,
        WorldTheme::Desert => Color::SANDY_BROWN,
        WorldTheme::Ice => Color::LIGHT_BLUE,
        WorldTheme::Volcano => Color::DARK_RED,
        WorldTheme::Space => Color::DARK_BLUE,
    };

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: bg_color,
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    // Spawn puddings
    for (i, pudding_data) in level_data.puddings.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: get_pudding_color(&pudding_data.pudding_type),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pudding_data.position.x, pudding_data.position.y, 0.0),
                ..default()
            },
            Pudding::new(i, pudding_data.pudding_type.clone()),
        ));
    }
}

fn handle_pudding_drag(
    mut game_state: ResMut<GameState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    pudding_q: Query<(Entity, &Transform, &Pudding)>,
    mut pudding_events: EventWriter<PuddingDragEvent>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(mouse_button) = window.cursor.pressed_buttons() {
        if mouse_button == MouseButton::Left {
            if !game_state.is_dragging {
                // Start dragging
                if let Some(cursor_pos) = window.cursor.position() {
                    if let Some(world_pos) = screen_to_world(cursor_pos, camera, camera_transform) {
                        game_state.is_dragging = true;
                        game_state.drag_start = Some(world_pos);

                        // Find pudding under cursor
                        for (entity, transform, _) in pudding_q.iter() {
                            let pudding_pos = transform.translation.truncate();
                            let pudding_size = Vec2::new(40.0, 40.0);
                            let half_size = pudding_size / 2.0;

                            if world_pos.x >= pudding_pos.x - half_size.x
                                && world_pos.x <= pudding_pos.x + half_size.x
                                && world_pos.y >= pudding_pos.y - half_size.y
                                && world_pos.y <= pudding_pos.y + half_size.y
                            {
                                game_state.selected_pudding = Some(entity);
                                break;
                            }
                        }
                    }
                }
            }
        }
    } else {
        if game_state.is_dragging {
            // End dragging
            game_state.is_dragging = false;
            if let (Some(start_pos), Some(selected)) = (game_state.drag_start, game_state.selected_pudding) {
                if let Some(cursor_pos) = window.cursor.position() {
                    if let Some(end_pos) = screen_to_world(cursor_pos, camera, camera_transform) {
                        let direction = end_pos - start_pos;
                        if direction.length() > 5.0 {
                            // Valid drag movement
                            pudding_events.send(PuddingDragEvent {
                                pudding: selected,
                                direction: direction.normalize(),
                            });
                        }
                    }
                }
            }
            game_state.drag_start = None;
            game_state.selected_pudding = None;
        }
    }
}

fn screen_to_world(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    camera.viewport_to_world_2d(camera_transform, screen_pos)
}

#[derive(Event)]
struct PuddingMergeEvent {
    pudding1: Entity,
    pudding2: Entity,
}

#[derive(Event)]
struct GameWinEvent;

fn handle_pudding_movement(
    mut pudding_events: EventReader<PuddingDragEvent>,
    mut pudding_q: Query<(Entity, &mut Pudding, &mut Transform)>,
    mut game_state: ResMut<GameState>,
) {
    // Handle telepathic movement for purple puddings
    let mut purple_dragged = false;
    let mut drag_entity = None;
    
    for event in pudding_events.read() {
        if let Ok((_, pudding, _)) = pudding_q.get_mut(event.pudding) {
            if pudding.pudding_type == PuddingType::Purple {
                purple_dragged = true;
                drag_entity = Some(event.pudding);
            }
        }
    }

    // Process all drag events
    for event in pudding_events.read() {
        if let Ok((entity, mut pudding, mut transform)) = pudding_q.get_mut(event.pudding) {
            if !pudding.is_moving && game_state.moves_remaining > 0 {
                pudding.is_moving = true;
                pudding.target_position = Some(transform.translation.truncate() + event.direction * 200.0);
                pudding.velocity = event.direction * 200.0;
                game_state.moves_remaining -= 1;
                
                // If this is a purple pudding, also move all other purple puddings
                if pudding.pudding_type == PuddingType::Purple {
                    for (other_entity, mut other_pudding, mut other_transform) in pudding_q.iter_mut() {
                        if other_pudding.pudding_type == PuddingType::Purple 
                           && other_entity != entity 
                           && !other_pudding.is_moving {
                            other_pudding.is_moving = true;
                            other_pudding.target_position = Some(
                                other_transform.translation.truncate() + event.direction * 200.0
                            );
                            other_pudding.velocity = event.direction * 200.0;
                        }
                    }
                }
            }
        }
    }

    // Update pudding positions
    for (_, mut pudding, mut transform) in pudding_q.iter_mut() {
        if pudding.is_moving {
            let target = pudding.target_position.unwrap();
            let current_pos = transform.translation.truncate();
            let remaining_distance = target - current_pos;
            
            if remaining_distance.length() < 10.0 {
                // Snap to target
                transform.translation.x = target.x;
                transform.translation.y = target.y;
                pudding.is_moving = false;
                pudding.velocity = Vec2::ZERO;
            } else {
                // Move towards target
                let move_amount = pudding.velocity.normalize() * 5.0;
                transform.translation.x += move_amount.x;
                transform.translation.y += move_amount.y;
            }
        }
    }
}

fn check_pudding_collisions(
    mut commands: Commands,
    mut pudding_q: Query<(Entity, &Transform, &mut Pudding)>, 
    mut merge_events: EventWriter<PuddingMergeEvent>,
) {
    let mut puddings: Vec<_> = pudding_q.iter().collect();
    let mut merged = Vec::new();
    
    for i in 0..puddings.len() {
        let (entity1, transform1, pudding1) = &puddings[i];
        if merged.contains(entity1) {
            continue;
        }
        
        for j in i+1..puddings.len() {
            let (entity2, transform2, pudding2) = &puddings[j];
            if merged.contains(entity2) {
                continue;
            }

            let pos1 = transform1.translation.truncate();
            let pos2 = transform2.translation.truncate();
            let distance = (pos1 - pos2).length();

            // Check collision (puddings are 40x40)
            if distance < 40.0 && !pudding1.is_moving && !pudding2.is_moving {
                // Merge the puddings
                merge_events.send(PuddingMergeEvent {
                    pudding1: *entity1,
                    pudding2: *entity2,
                });
                merged.push(*entity1);
                merged.push(*entity2);
                break;
            }
        }
    }
}

fn check_win_condition(
    pudding_q: Query<&Pudding>,
    level: Res<CurrentLevel>,
    collected_stars: Res<CollectedStars>,
    mut win_events: EventWriter<GameWinEvent>,
) {
    let pudding_count = pudding_q.iter().count();
    let stars_needed = level.data.required_stars;
    let stars_collected = collected_stars.stars.iter().filter(|&&collected| collected).count();

    // Win if only one pudding remains and required stars are collected
    if pudding_count == 1 && stars_collected >= stars_needed {
        win_events.send(GameWinEvent);
    }
}

fn handle_level_transition(
    mut win_events: EventReader<GameWinEvent>,
    mut level: ResMut<CurrentLevel>,
    mut world: ResMut<CurrentWorld>,
    mut game_state: ResMut<GameState>,
    mut collected_stars: ResMut<CollectedStars>,
    mut pudding_q: Query<Entity, With<Pudding>>,
    mut star_q: Query<Entity, With<Star>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in win_events.read() {
        // Clear current level
        for entity in pudding_q.iter() {
            commands.entity(entity).despawn();
        }
        for entity in star_q.iter() {
            commands.entity(entity).despawn();
        }

        // Move to next level
        level.index += 1;
        if level.index < WORLDS[world.index].levels.len() {
            level.data = WORLDS[world.index].levels[level.index].clone();
            game_state.moves_remaining = level.data.max_moves;
            game_state.stars_collected = 0;
            collected_stars.stars = vec![false; level.data.stars.len()];

            // Spawn new level
            spawn_level(&mut commands, &mut meshes, &mut materials, &level.data);
            star::setup_stars(&mut commands, &level.data.stars);
        } else {
            // Move to next world
            world.index += 1;
            if world.index < WORLDS.len() {
                level.index = 0;
                level.data = WORLDS[world.index].levels[0].clone();
                game_state.moves_remaining = level.data.max_moves;
                game_state.stars_collected = 0;
                collected_stars.stars = vec![false; level.data.stars.len()];

                // Spawn new world's first level
                spawn_level(&mut commands, &mut meshes, &mut materials, &level.data);
                star::setup_stars(&mut commands, &level.data.stars);
            } else {
                // All worlds completed
                println!("All worlds completed!");
            }
        }
    }
}