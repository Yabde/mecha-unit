use bevy::prelude::*;

/// Zone occupée par la minimap (308x308 pixels, placée à 10px du bord bas-droit)
pub fn is_over_minimap(window: &Window, cursor_position: Vec2) -> bool {
    let minimap_rect = Rect::from_corners(
        Vec2::new(window.width() - 318.0, window.height() - 318.0),
        Vec2::new(window.width(), window.height()),
    );
    minimap_rect.contains(cursor_position)
}

/// Vérifie si la souris survole un élément d'interface Bevy UI (bouton, panneau, etc.)
pub fn is_ui_blocking(q_interactions: &Query<&Interaction, With<Node>>) -> bool {
    for interaction in q_interactions.iter() {
        if *interaction != Interaction::None {
            return true;
        }
    }
    false
}
