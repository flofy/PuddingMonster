use bevy::prelude::*;
use bevy::window::WindowMode;

mod pudding;
mod levels;
mod ui;

use pudding::*;
use levels::*;
use ui::*;

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
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .init_resource::<CurrentLevel>()
            .add_systems(Startup, setup_game)
            .add_systems(Update, (
                handle_pudding_drag,
                handle_pudding_movement,
                check_pudding_collisions,
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
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            moves_remaining: 0,
            is_dragging: false,
            drag_start: None,
            selected_pudding: None,
        }
    }
}

#[derive(Resource)]
struct CurrentLevel {
    index: usize,
    data: LevelData,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            index: 0,
            data: LEVELS[0].clone(),
        }
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level: ResMut<CurrentLevel>,
    mut game_state: ResMut<GameState>,
) {
    // Setup camera
    commands.spawn(Camera2dBundle::default());

    // Load first level
    *level = CurrentLevel {
        index: 0,
        data: LEVELS[0].clone(),
    };
    game_state.moves_remaining = level.data.max_moves;

    // Spawn puddings for the first level
    spawn_level(&mut commands, &mut meshes, &mut materials, &level.data);

    // Setup UI
    ui::setup_ui(&mut commands);
}

fn spawn_level(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    level_data: &LevelData,
) {
    // Spawn board background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    // Spawn puddings
    for (i, pudding_data) in level_data.puddings.iter().enumerate() {
        let entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: pudding_data.color,
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pudding_data.position.x, pudding_data.position.y, 0.0),
                ..default()
            },
            Pudding {
                id: i,
                pudding_type: pudding_data.pudding_type.clone(),
                is_moving: false,
                target_position: None,
                velocity: Vec2::ZERO,
            },
        )).id();

        // Store entity reference if needed
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

    if let Some(mouse_button) = window.cursor.pressed buttons() {
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
struct PuddingDragEvent {
    pudding: Entity,
    direction: Vec2,
}

fn handle_pudding_movement(
    mut pudding_events: EventReader<PuddingDragEvent>,
    mut pudding_q: Query<(&mut Pudding, &mut Transform)>,
    mut game_state: ResMut<GameState>,
) {
    for event in pudding_events.read() {
        if let Ok((mut pudding, mut transform)) = pudding_q.get_mut(event.pudding) {
            if !pudding.is_moving && game_state.moves_remaining > 0 {
                pudding.is_moving = true;
                pudding.target_position = Some(transform.translation.truncate() + event.direction * 200.0);
                pudding.velocity = event.direction * 200.0;
                game_state.moves_remaining -= 1;
            }
        }
    }

    // Update pudding positions
    for (mut pudding, mut transform) in pudding_q.iter_mut() {
        if pudding.is_moving {
            let remaining_distance = pudding.target_position.unwrap() - transform.translation.truncate();
            if remaining_distance.length() < 10.0 {
                // Snap to target
                transform.translation.x = pudding.target_position.unwrap().x;
                transform.translation.y = pudding.target_position.unwrap().y;
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
    mut pudding_q: Query<(Entity, &Transform, &mut Pudding)>, 
    mut commands: Commands,
    mut merge_events: EventWriter<PuddingMergeEvent>,
) {
    let mut puddings: Vec<_> = pudding_q.iter().collect();
    
    for i in 0..puddings.len() {
        for j in i+1..puddings.len() {
            let (entity1, transform1, pudding1) = &puddings[i];
            let (entity2, transform2, pudding2) = &puddings[j];

            let pos1 = transform1.translation.truncate();
            let pos2 = transform2.translation.truncate();
            let distance = (pos1 - pos2).length();

            // Check collision (puddings are 40x40)
            if distance < 40.0 && !pudding1.is_moving && !pudding2.is_moving {
                // Simple merge logic - could be enhanced based on pudding types
                merge_events.send(PuddingMergeEvent {
                    pudding1: *entity1,
                    pudding2: *entity2,
                });
            }
        }
    }
}

#[derive(Event)]
struct PuddingMergeEvent {
    pudding1: Entity,
    pudding2: Entity,
}

fn check_win_condition(
    pudding_q: Query<&Pudding>,
    level: Res<CurrentLevel>,
    mut win_events: EventWriter<GameWinEvent>,
) {
    if pudding_q.iter().count() == 1 {
        win_events.send(GameWinEvent);
    }
}

#[derive(Event)]
struct GameWinEvent;

fn handle_level_transition(
    mut win_events: EventReader<GameWinEvent>,
    mut level: ResMut<CurrentLevel>,
    mut game_state: ResMut<GameState>,
    mut pudding_q: Query<Entity, With<Pudding>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in win_events.read() {
        // Clear current level
        for entity in pudding_q.iter() {
            commands.entity(entity).despawn();
        }

        // Move to next level
        level.index += 1;
        if level.index < LEVELS.len() {
            level.data = LEVELS[level.index].clone();
            game_state.moves_remaining = level.data.max_moves;

            // Spawn new level
            spawn_level(&mut commands, &mut meshes, &mut materials, &level.data);
        } else {
            // Game completed
            println!("All levels completed!");
        }
    }
}