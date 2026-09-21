use bevy::prelude::*;

#[derive(Component)]
pub struct Pudding {
    pub id: usize,
    pub pudding_type: PuddingType,
    pub is_moving: bool,
    pub target_position: Option<Vec2>,
    pub velocity: Vec2,
}

#[derive(Clone, Debug)]
pub enum PuddingType {
    Normal,
    Green,  // Leaves slime trail
    Purple, // Telepathically connected
}

impl Pudding {
    pub fn new(id: usize, pudding_type: PuddingType) -> Self {
        Self {
            id,
            pudding_type,
            is_moving: false,
            target_position: None,
            velocity: Vec2::ZERO,
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