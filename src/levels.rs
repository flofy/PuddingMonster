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
    pub stars: Vec<Vec2>, // Positions of star collectibles
    pub required_stars: usize, // Number of stars needed to complete (0 = all)
}

#[derive(Clone)]
pub struct WorldData {
    pub name: String,
    pub levels: Vec<LevelData>,
    pub theme: WorldTheme,
}

#[derive(Clone, Debug)]
pub enum WorldTheme {
    Forest,
    Desert,
    Ice,
    Volcano,
    Space,
}

// Helper function to generate circular arrangements
fn circle_positions(center: Vec2, radius: f32, count: usize) -> Vec<Vec2> {
    (0..count).map(|i| {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (count as f32);
        Vec2::new(center.x + radius * angle.cos(), center.y + radius * angle.sin())
    }).collect()
}

// Helper function to generate grid positions
fn grid_positions(start: Vec2, spacing: f32, rows: usize, cols: usize) -> Vec<Vec2> {
    let mut positions = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            positions.push(Vec2::new(start.x + col as f32 * spacing, start.y + row as f32 * spacing));
        }
    }
    positions
}

pub const WORLDS: &[WorldData] = &[
    // World 1: Forest
    WorldData {
        name: "Forest World".to_string(),
        theme: WorldTheme::Forest,
        levels: vec![
            // Level 1-1: Tutorial
            LevelData {
                name: "Forest Tutorial".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(200.0, 200.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(300.0, 200.0), pudding_type: PuddingType::Normal },
                ],
                max_moves: 5,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(250.0, 150.0)],
                required_stars: 0,
            },
            // Level 1-2: Three puddings
            LevelData {
                name: "Triple Trouble".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(150.0, 150.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(300.0, 150.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(225.0, 300.0), pudding_type: PuddingType::Normal },
                ],
                max_moves: 8,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0)],
                required_stars: 0,
            },
            // Level 1-3: Green pudding intro
            LevelData {
                name: "Sticky Start".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(150.0, 100.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(350.0, 100.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(250.0, 300.0), pudding_type: PuddingType::Normal },
                ],
                max_moves: 10,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0), Vec2::new(300.0, 200.0)],
                required_stars: 1,
            },
            // Level 1-4: Purple pudding intro
            LevelData {
                name: "Telepathic Test".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(300.0, 100.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(200.0, 300.0), pudding_type: PuddingType::Normal },
                ],
                max_moves: 12,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 150.0)],
                required_stars: 0,
            },
            // Level 1-5: Mix of types
            LevelData {
                name: "Forest Mix".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(200.0, 100.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(300.0, 100.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(150.0, 300.0), pudding_type: PuddingType::Normal },
                ],
                max_moves: 15,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(150.0, 200.0), Vec2::new(250.0, 200.0)],
                required_stars: 1,
            },
            // Level 1-6: Circle arrangement
            LevelData {
                name: "Forest Circle".to_string(),
                puddings: circle_positions(Vec2::new(250.0, 250.0), 100.0, 4)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: match i % 3 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    })
                    .collect(),
                max_moves: 18,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(250.0, 250.0)],
                required_stars: 0,
            },
            // Level 1-7: More puddings
            LevelData {
                name: "Forest Gathering".to_string(),
                puddings: grid_positions(Vec2::new(100.0, 100.0), 80.0, 2, 3)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: match i % 2 {
                            0 => PuddingType::Normal,
                            _ => PuddingType::Green,
                        }
                    })
                    .collect(),
                max_moves: 20,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0), Vec2::new(250.0, 250.0)],
                required_stars: 1,
            },
            // Level 1-8: Complex arrangement
            LevelData {
                name: "Forest Maze".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(100.0, 100.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(300.0, 100.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(100.0, 300.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(300.0, 300.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(200.0, 200.0), pudding_type: PuddingType::Normal },
                ],
                max_moves: 22,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(150.0, 150.0), Vec2::new(250.0, 250.0)],
                required_stars: 2,
            },
            // Level 1-9 to 1-25: Generated levels
            LevelData {
                name: "Forest Challenge 1".to_string(),
                puddings: grid_positions(Vec2::new(100.0, 50.0), 70.0, 3, 3)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: match i % 3 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    })
                    .collect(),
                max_moves: 25,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 150.0), Vec2::new(250.0, 200.0), Vec2::new(300.0, 150.0)],
                required_stars: 2,
            },
            LevelData {
                name: "Forest Challenge 2".to_string(),
                puddings: circle_positions(Vec2::new(250.0, 200.0), 120.0, 6)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: match i % 2 {
                            0 => PuddingType::Green,
                            _ => PuddingType::Normal,
                        }
                    })
                    .collect(),
                max_moves: 28,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 150.0), Vec2::new(300.0, 150.0)],
                required_stars: 1,
            },
            LevelData {
                name: "Forest Challenge 3".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(50.0, 50.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(150.0, 50.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(250.0, 50.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(50.0, 250.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(150.0, 250.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(250.0, 250.0), pudding_type: PuddingType::Normal },
                ],
                max_moves: 30,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(100.0, 150.0), Vec2::new(200.0, 150.0), Vec2::new(200.0, 250.0)],
                required_stars: 2,
            },
            // Add more levels...
            LevelData {
                name: "Forest Challenge 4".to_string(),
                puddings: grid_positions(Vec2::new(80.0, 80.0), 60.0, 4, 3)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: match i % 4 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            2 => PuddingType::Purple,
                            _ => PuddingType::Normal,
                        }
                    })
                    .collect(),
                max_moves: 35,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 150.0), Vec2::new(250.0, 200.0)],
                required_stars: 2,
            },
            LevelData {
                name: "Forest Challenge 5".to_string(),
                puddings: circle_positions(Vec2::new(250.0, 250.0), 80.0, 8)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: match i % 3 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    })
                    .collect(),
                max_moves: 32,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0), Vec2::new(300.0, 200.0)],
                required_stars: 2,
            },
            // Levels 11-25: More complex patterns
            LevelData {
                name: "Forest Spiral".to_string(),
                puddings: (0..8).map(|i| {
                    let angle = 2.0 * std::f32::consts::PI * (i as f32) / 8.0;
                    let radius = 50.0 + (i as f32) * 30.0;
                    PuddingData {
                        position: Vec2::new(250.0 + radius * angle.cos(), 250.0 + radius * angle.sin()),
                        pudding_type: match i % 3 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    }
                }).collect(),
                max_moves: 40,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0), Vec2::new(300.0, 200.0), Vec2::new(250.0, 300.0)],
                required_stars: 2,
            },
            LevelData {
                name: "Forest Cross".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(200.0, 100.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(200.0, 300.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(100.0, 200.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(300.0, 200.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(200.0, 200.0), pudding_type: PuddingType::Purple },
                ],
                max_moves: 25,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(150.0, 150.0), Vec2::new(250.0, 150.0)],
                required_stars: 2,
            },
            LevelData {
                name: "Forest Star".to_string(),
                puddings: circle_positions(Vec2::new(250.0, 250.0), 100.0, 5)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: if i == 0 { PuddingType::Purple } else { PuddingType::Green }
                    })
                    .collect(),
                max_moves: 22,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0), Vec2::new(300.0, 200.0), Vec2::new(250.0, 300.0)],
                required_stars: 3,
            },
            LevelData {
                name: "Forest Triangle".to_string(),
                puddings: vec![
                    PuddingData { position: Vec2::new(150.0, 100.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(350.0, 100.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(250.0, 300.0), pudding_type: PuddingType::Purple },
                    PuddingData { position: Vec2::new(100.0, 200.0), pudding_type: PuddingType::Normal },
                    PuddingData { position: Vec2::new(400.0, 200.0), pudding_type: PuddingType::Green },
                    PuddingData { position: Vec2::new(250.0, 400.0), pudding_type: PuddingType::Purple },
                ],
                max_moves: 30,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0), Vec2::new(300.0, 200.0)],
                required_stars: 2,
            },
            // Continue with more levels to reach 25...
            LevelData {
                name: "Forest Big Mix".to_string(),
                puddings: grid_positions(Vec2::new(70.0, 70.0), 50.0, 5, 4)
                    .into_iter()
                    .enumerate()
                    .map(|(i, pos)| PuddingData {
                        position: pos,
                        pudding_type: match i % 3 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    })
                    .collect(),
                max_moves: 45,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 150.0), Vec2::new(300.0, 150.0), Vec2::new(250.0, 250.0)],
                required_stars: 2,
            },
            LevelData {
                name: "Forest Final".to_string(),
                puddings: (0..10).map(|i| {
                    let angle = 2.0 * std::f32::consts::PI * (i as f32) / 10.0;
                    let radius = 80.0 + (i as f32) * 20.0;
                    PuddingData {
                        position: Vec2::new(250.0 + radius * angle.cos(), 250.0 + radius * angle.sin()),
                        pudding_type: match i % 3 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    }
                }).collect(),
                max_moves: 50,
                board_size: Vec2::new(800.0, 600.0),
                stars: vec![Vec2::new(200.0, 200.0), Vec2::new(300.0, 200.0), Vec2::new(250.0, 300.0)],
                required_stars: 3,
            },
        ],
    },
    
    // World 2: Desert (10 levels for brevity, will expand to 25)
    WorldData {
        name: "Desert World".to_string(),
        theme: WorldTheme::Desert,
        levels: (0..25).map(|i| {
            let pudding_count = 3 + (i * 2).min(15);
            let max_moves = 10 + i * 3;
            let stars_count = ((i + 3) / 5).min(3);
            
            LevelData {
                name: format!("Desert Level {}", i + 1),
                puddings: grid_positions(Vec2::new(100.0, 100.0), 40.0, (pudding_count + 1) / 2, 2)
                    .into_iter()
                    .enumerate()
                    .map(|(j, pos)| PuddingData {
                        position: pos,
                        pudding_type: match j % 3 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    })
                    .take(pudding_count)
                    .collect(),
                max_moves,
                board_size: Vec2::new(800.0, 600.0),
                stars: (0..stars_count).map(|s| Vec2::new(150.0 + s as f32 * 100.0, 100.0)).collect(),
                required_stars: stars_count,
            }
        }).collect(),
    },
    
    // World 3: Ice
    WorldData {
        name: "Ice World".to_string(),
        theme: WorldTheme::Ice,
        levels: (0..25).map(|i| {
            let pudding_count = 4 + (i * 2).min(16);
            let max_moves = 12 + i * 3;
            let stars_count = ((i + 4) / 6).min(3);
            
            LevelData {
                name: format!("Ice Level {}", i + 1),
                puddings: circle_positions(Vec2::new(250.0, 250.0), 50.0 + i as f32 * 5.0, pudding_count)
                    .into_iter()
                    .enumerate()
                    .map(|(j, pos)| PuddingData {
                        position: pos,
                        pudding_type: match j % 4 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            2 => PuddingType::Purple,
                            _ => PuddingType::Normal,
                        }
                    })
                    .collect(),
                max_moves,
                board_size: Vec2::new(800.0, 600.0),
                stars: (0..stars_count).map(|s| Vec2::new(200.0 + s as f32 * 50.0, 150.0)).collect(),
                required_stars: stars_count,
            }
        }).collect(),
    },
    
    // World 4: Volcano
    WorldData {
        name: "Volcano World".to_string(),
        theme: WorldTheme::Volcano,
        levels: (0..25).map(|i| {
            let pudding_count = 5 + (i * 2).min(18);
            let max_moves = 15 + i * 3;
            let stars_count = ((i + 5) / 7).min(3);
            
            LevelData {
                name: format!("Volcano Level {}", i + 1),
                puddings: grid_positions(Vec2::new(50.0, 50.0), 35.0, (pudding_count + 2) / 3, 3)
                    .into_iter()
                    .enumerate()
                    .map(|(j, pos)| PuddingData {
                        position: pos,
                        pudding_type: match j % 2 {
                            0 => PuddingType::Normal,
                            _ => PuddingType::Purple,
                        }
                    })
                    .take(pudding_count)
                    .collect(),
                max_moves,
                board_size: Vec2::new(800.0, 600.0),
                stars: (0..stars_count).map(|s| Vec2::new(100.0 + s as f32 * 100.0, 200.0)).collect(),
                required_stars: stars_count,
            }
        }).collect(),
    },
    
    // World 5: Space
    WorldData {
        name: "Space World".to_string(),
        theme: WorldTheme::Space,
        levels: (0..25).map(|i| {
            let pudding_count = 6 + (i * 2).min(20);
            let max_moves = 20 + i * 4;
            let stars_count = ((i + 6) / 8).min(3);
            
            LevelData {
                name: format!("Space Level {}", i + 1),
                puddings: (0..pudding_count).map(|j| {
                    let angle = 2.0 * std::f32::consts::PI * (j as f32) / pudding_count as f32;
                    let radius = 60.0 + (j as f32) * 15.0;
                    PuddingData {
                        position: Vec2::new(250.0 + radius * angle.cos(), 250.0 + radius * angle.sin()),
                        pudding_type: match j % 5 {
                            0 => PuddingType::Normal,
                            1 => PuddingType::Green,
                            2 => PuddingType::Purple,
                            3 => PuddingType::Green,
                            _ => PuddingType::Purple,
                        }
                    }
                }).collect(),
                max_moves,
                board_size: Vec2::new(800.0, 600.0),
                stars: (0..stars_count).map(|s| Vec2::new(150.0 + s as f32 * 100.0, 100.0)).collect(),
                required_stars: stars_count,
            }
        }).collect(),
    },
];

pub fn get_total_levels() -> usize {
    WORLDS.iter().map(|world| world.levels.len()).sum()
}

pub fn get_level(world_index: usize, level_index: usize) -> Option<&'static LevelData> {
    WORLDS.get(world_index).and_then(|world| world.levels.get(level_index))
}