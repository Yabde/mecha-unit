pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::{setup_minimap, update_minimap_viewport};

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_minimap)
           .add_systems(Update, update_minimap_viewport);
    }
}
