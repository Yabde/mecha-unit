pub mod components;
pub mod resources;
pub mod systems;
pub mod production; // NEW

use bevy::prelude::*;
use resources::PlacementState;
use systems::*;
use production::*; // NEW

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlacementState>()
           .add_systems(Update, (handle_placement, process_production_queues, draw_production_progress)); // Ajout de l'UI
    }
}
