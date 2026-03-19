pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, update_economy_ui);
    }
}
