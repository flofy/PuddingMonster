use bevy::prelude::*;

#[derive(Component)]
pub struct Pudding {
    pub id: usize,
    pub pudding_type: PuddingType,
    pub is_moving: bool,
    pub target_position: Option<Vec2>,
    pub velocity: Vec2,
    pub size: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PuddingType {
    Normal,
    Green,  // Leaves slime trail
    Purple, // Telepathically connected
}

impl Pudding {
    pub fn new(id: usize, pudding_type: PuddingType) -> Self {
        Self {
            id,
            pudding_type: pudding_type.clone(),
            is_moving: false,
            target_position: None,
            velocity: Vec2::ZERO,
            size: 40.0,
        }
    }
}

pub fn get_pudding_color(pudding_type: &PuddingType) -> Color {
    match pudding_type {
        PuddingType::Normal => Color::rgb(0.8, 0.2, 0.2), // Red
        PuddingType::Green => Color::rgb(0.2, 0.8, 0.2), // Green
        PuddingType::Purple => Color::rgb(0.5, 0.2, 0.8), // Purple
    }
}

// System to handle slime trail from green puddings
pub fn handle_slime_trail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pudding_q: Query<(Entity, &Transform, &Pudding), Changed<Transform>>,
    trail_q: Query<Entity, With<SlimeTrail>>,
) {
    for (entity, transform, pudding) in pudding_q.iter() {
        if pudding.pudding_type == PuddingType::Green && pudding.is_moving {
            // Create slime trail at current position
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.2, 0.8, 0.2, 0.5),
                        custom_size: Some(Vec2::new(20.0, 20.0)),
                        ..default()
                    },
                    transform: *transform,
                    ..default()
                },
                SlimeTrail {
                    lifetime: 100, // Frames to live
                },
            ));
        }
    }

    // Clean up old trails
    for entity in trail_q.iter() {
        // This would be handled by a lifetime system in a real implementation
    }
}

#[derive(Component)]
pub struct SlimeTrail {
    pub lifetime: u32,
}

// System to handle purple pudding telepathic movement
pub fn handle_telepathic_movement(
    mut pudding_q: Query<(&mut Pudding, &mut Transform)>, 
    mut drag_events: EventReader<PuddingDragEvent>,
) {
    for event in drag_events.read() {
        // Find the dragged pudding
        if let Ok((pudding, _)) = pudding_q.get(event.pudding) {
            if pudding.pudding_type == PuddingType::Purple {
                // Find all other purple puddings and move them in the same direction
                for (mut other_pudding, mut other_transform) in pudding_q.iter_mut() {
                    if other_pudding.pudding_type == PuddingType::Purple 
                       && other_pudding.id != pudding.id 
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

#[derive(Event)]
pub struct PuddingDragEvent {
    pub pudding: Entity,
    pub direction: Vec2,
}
