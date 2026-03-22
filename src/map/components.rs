use bevy::prelude::*;

/// Limites du monde de jeu
#[derive(Resource)]
pub struct MapBounds {
    pub width: f32,
    pub height: f32,
}

impl Default for MapBounds {
    fn default() -> Self {
        Self {
            width: 4000.0,
            height: 3000.0,
        }
    }
}
