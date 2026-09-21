use crate::pudding::PuddingType;
use bevy::prelude::Vec2;

#[derive(Clone)]
pub struct PuddingData {
    pub position: Vec2,
    pub pudding_type: PuddingType,
}

#[derive(Clone)]
pub struct LevelData {
    pub name: String,
    pub puddings: Vec<PuddingData>,
    pub max_moves: u32,
    pub board_size: Vec2,
}

pub const LEVELS: &[LevelData] = &[
    // Level 1: Basic introduction
    LevelData {
        name: "Tutorial".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(200.0, 100.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 5,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 2: Three puddings
    LevelData {
        name: "Three Puddings".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(200.0, 100.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(150.0, 200.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 8,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 3: Introduce Green pudding
    LevelData {
        name: "Sticky Situation".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(300.0, 100.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(200.0, 300.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 10,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 4: Introduce Purple puddings
    LevelData {
        name: "Telepathic Twist".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(300.0, 100.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(200.0, 300.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 12,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 5: Mix of all types
    LevelData {
        name: "Pudding Party".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(200.0, 100.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(300.0, 100.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(150.0, 300.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(250.0, 300.0), pudding_type: PuddingType::Purple },
        ],
        max_moves: 15,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 6: Challenge level - More puddings
    LevelData {
        name: "Pudding Overload".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(50.0, 50.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(150.0, 50.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(250.0, 50.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(100.0, 200.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(200.0, 200.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(150.0, 350.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 20,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 7: Additional custom level - Complex arrangement
    LevelData {
        name: "The Maze".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(300.0, 100.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(100.0, 300.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(300.0, 300.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(200.0, 200.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 18,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 8: Additional custom level - All green
    LevelData {
        name: "Sticky Business".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(200.0, 100.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(150.0, 200.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(250.0, 200.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 14,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 9: Additional custom level - Circle arrangement
    LevelData {
        name: "Circle of Pudding".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(200.0, 100.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(300.0, 200.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(200.0, 300.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(100.0, 200.0), pudding_type: PuddingType::Purple },
        ],
        max_moves: 16,
        board_size: Vec2::new(800.0, 600.0),
    },
    
    // Level 10: Final challenge
    LevelData {
        name: "Final Feast".to_string(),
        puddings: vec![
            PuddingData { position: Vec2::new(50.0, 50.0), pudding_type: PuddingType::Normal },
            PuddingData { position: Vec2::new(150.0, 50.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(250.0, 50.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(50.0, 250.0), pudding_type: PuddingType::Purple },
            PuddingData { position: Vec2::new(150.0, 250.0), pudding_type: PuddingType::Green },
            PuddingData { position: Vec2::new(250.0, 250.0), pudding_type: PuddingType::Normal },
        ],
        max_moves: 25,
        board_size: Vec2::new(800.0, 600.0),
    },
];