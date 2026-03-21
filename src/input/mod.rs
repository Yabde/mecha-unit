pub mod resources;
pub mod helpers;
pub mod selection;
pub mod movement;

use bevy::prelude::*;
use resources::SelectionState;
use selection::*;
use movement::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
           .add_systems(Update, (
               handle_selection_input,
               draw_selection_box,
               handle_movement_orders,
           ));
    }
}
