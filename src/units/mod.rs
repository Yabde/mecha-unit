pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::{setup_units, animate_selection, move_units, handle_separation};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_units)
           .add_systems(Update, (animate_selection, move_units, handle_separation));
    }
}
