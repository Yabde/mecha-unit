pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::PlacementState;
use systems::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlacementState>()
           .add_systems(Update, handle_placement); // Priorité classique
    }
}
