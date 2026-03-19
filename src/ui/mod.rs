pub mod components;
pub mod systems;
pub mod build_menu; // NEW

use bevy::prelude::*;
use systems::*;
use build_menu::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui, setup_build_menu))
           .add_systems(Update, (update_economy_ui, update_build_menu_visibility, handle_build_button_clicks));
    }
}
