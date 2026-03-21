pub mod theme;
pub mod widgets;
pub mod hud;
pub mod panels;

use bevy::prelude::*;

use hud::economy_bar::*;
use panels::build_menu::*;
use panels::production_menu::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_economy_bar, setup_build_menu, setup_production_menu))
           .add_systems(Update, (
               update_economy_bar,
               update_build_menu_visibility, handle_build_button_clicks,
               update_production_menu_visibility, handle_production_button_clicks,
           ));
    }
}
