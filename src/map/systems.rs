use bevy::prelude::*;
use super::components::MapBounds;

/// Dessine les bordures de la carte en blanc
pub fn draw_map_borders(
    mut gizmos: Gizmos,
    bounds: Res<MapBounds>,
) {
    let hw = bounds.width / 2.0;
    let hh = bounds.height / 2.0;

    let color = Color::srgba(1.0, 1.0, 1.0, 0.4);

    // Les 4 cotes du rectangle
    gizmos.line_2d(Vec2::new(-hw, -hh), Vec2::new(hw, -hh), color);
    gizmos.line_2d(Vec2::new(hw, -hh), Vec2::new(hw, hh), color);
    gizmos.line_2d(Vec2::new(hw, hh), Vec2::new(-hw, hh), color);
    gizmos.line_2d(Vec2::new(-hw, hh), Vec2::new(-hw, -hh), color);
}
