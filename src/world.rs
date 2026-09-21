use bevy::prelude::*;
use crate::levels::*;

#[derive(Resource)]
pub struct CurrentWorld {
    pub index: usize,
}

impl Default for CurrentWorld {
    fn default() -> Self {
        Self { index: 0 }
    }
}

#[derive(Resource)]
pub struct UnlockedWorlds {
    pub unlocked: Vec<bool>,
}

impl Default for UnlockedWorlds {
    fn default() -> Self {
        let mut unlocked = vec![false; WORLDS.len()];
        if !unlocked.is_empty() {
            unlocked[0] = true; // First world is always unlocked
        }
        Self { unlocked }
    }
}

impl UnlockedWorlds {
    pub fn unlock(&mut self, world_index: usize) {
        if world_index < self.unlocked.len() {
            self.unlocked[world_index] = true;
        }
    }

    pub fn is_unlocked(&self, world_index: usize) -> bool {
        world_index < self.unlocked.len() && self.unlocked[world_index]
    }
}
