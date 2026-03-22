use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use super::components::MapBounds;
use crate::MainCamera;

const EDGE_SCROLL_MARGIN: f32 = 20.0;
const EDGE_SCROLL_SPEED: f32 = 500.0;
const ZOOM_SPEED: f32 = 0.1;
const ZOOM_MIN: f32 = 0.5;
const ZOOM_MAX: f32 = 2.5;

/// Deplace la camera quand la souris touche les bords de la fenetre
pub fn edge_scroll(
    time: Res<Time>,
    window: Single<&Window>,
    mut camera_query: Query<(&mut Transform, &Projection), With<MainCamera>>,
    bounds: Res<MapBounds>,
) {
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((mut transform, projection)) = camera_query.single_mut() else { return; };

    let scale = match projection {
        Projection::Orthographic(ortho) => ortho.scale,
        _ => 1.0,
    };

    let mut direction = Vec2::ZERO;

    if cursor_pos.x < EDGE_SCROLL_MARGIN { direction.x -= 1.0; }
    if cursor_pos.x > window.width() - EDGE_SCROLL_MARGIN { direction.x += 1.0; }
    if cursor_pos.y < EDGE_SCROLL_MARGIN { direction.y += 1.0; }
    if cursor_pos.y > window.height() - EDGE_SCROLL_MARGIN { direction.y -= 1.0; }

    if direction != Vec2::ZERO {
        let movement = direction.normalize() * EDGE_SCROLL_SPEED * scale * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }

    clamp_camera(&mut transform, &bounds, scale, window.width(), window.height());
}

/// Deplace la camera en maintenant le clic molette (middle-click drag)
pub fn middle_click_drag(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    mut camera_query: Query<(&mut Transform, &Camera, &Projection, &GlobalTransform), With<MainCamera>>,
    bounds: Res<MapBounds>,
) {
    if !buttons.pressed(MouseButton::Middle) { return; }

    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((mut transform, _camera, projection, _gt)) = camera_query.single_mut() else { return; };

    let scale = match projection {
        Projection::Orthographic(ortho) => ortho.scale,
        _ => 1.0,
    };

    // On utilise le delta entre la position actuelle et le centre de l'ecran
    let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let delta = cursor_pos - center;

    // Si le curseur est pres du centre, pas de mouvement (zone morte)
    if delta.length() < 5.0 { return; }

    // Plus le curseur est loin du centre, plus le mouvement est rapide
    let speed = delta * 2.0 * scale;
    transform.translation.x += speed.x * 0.016; // ~60fps approx
    transform.translation.y -= speed.y * 0.016; // Y inverse

    clamp_camera(&mut transform, &bounds, scale, window.width(), window.height());
}

/// Zoom/Dezoom avec la molette
pub fn zoom_camera(
    mut scroll_events: MessageReader<MouseWheel>,
    mut camera_query: Query<&mut Projection, With<MainCamera>>,
) {
    for event in scroll_events.read() {
        let Ok(mut projection) = camera_query.single_mut() else { return; };
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale -= event.y * ZOOM_SPEED;
            ortho.scale = ortho.scale.clamp(ZOOM_MIN, ZOOM_MAX);
        }
    }
}

/// Empeche la camera de sortir des limites de la carte
fn clamp_camera(transform: &mut Transform, bounds: &MapBounds, scale: f32, win_w: f32, win_h: f32) {
    let half_view_w = (win_w / 2.0) * scale;
    let half_view_h = (win_h / 2.0) * scale;
    let hw = bounds.width / 2.0;
    let hh = bounds.height / 2.0;

    if half_view_w < hw {
        transform.translation.x = transform.translation.x.clamp(-hw + half_view_w, hw - half_view_w);
    }
    if half_view_h < hh {
        transform.translation.y = transform.translation.y.clamp(-hh + half_view_h, hh - half_view_h);
    }
}
