use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerEconomy {
    pub crystals: u32,
}

impl Default for PlayerEconomy {
    fn default() -> Self {
        Self { crystals: 1000 }
    }
}
