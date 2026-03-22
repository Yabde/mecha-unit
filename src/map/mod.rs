pub mod components;
pub mod systems;
pub mod camera;

use bevy::prelude::*;
use components::MapBounds;
use systems::draw_map_borders;
use camera::{edge_scroll, middle_click_drag, zoom_camera};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapBounds>()
           .add_systems(Update, (draw_map_borders, edge_scroll, middle_click_drag, zoom_camera));
    }
}
