use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerEconomy {
    pub crystals: u32,
}
