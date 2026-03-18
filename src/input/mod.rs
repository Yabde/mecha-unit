pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::SelectionState;
use systems::{handle_selection_input, draw_selection_box, handle_movement_orders};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>();
        app.add_systems(Update, (
            handle_selection_input,
            draw_selection_box, 
            handle_movement_orders
        ));
    }
}
