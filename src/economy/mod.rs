pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::PlayerEconomy;
use systems::*;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerEconomy>()
           .add_systems(Startup, spawn_economy_entities)
           .add_systems(Update, worker_ai);
    }
}
