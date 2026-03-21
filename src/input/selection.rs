use bevy::prelude::*;
use crate::input::resources::SelectionState;
use crate::input::helpers::{is_over_minimap, is_ui_blocking};
use crate::core::components::{Selected, SelectionCollider};
use crate::building::resources::PlacementState;

pub fn handle_selection_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mut selection_state: ResMut<SelectionState>,
    q_units: Query<(Entity, &Transform, &SelectionCollider, Option<&Selected>)>,
    placement_state: Res<PlacementState>,
    q_interactions: Query<&Interaction, With<Node>>,
) {
    if placement_state.active_building.is_some() { return; }
    if is_ui_blocking(&q_interactions) { return; }

    let Some(cursor_position) = window.cursor_position() else { return; };
    let (camera, camera_transform) = *camera_query;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    let is_over_minimap = is_over_minimap(&window, cursor_position);

    // 1. Début de la sélection (On clique)
    if buttons.just_pressed(MouseButton::Left) && !is_over_minimap {
        selection_state.start_pos = Some(world_pos);
        selection_state.end_pos = Some(world_pos);
    }

    // 2. Mise à jour de la sélection (On maintient et on glisse)
    if buttons.pressed(MouseButton::Left) {
        if selection_state.start_pos.is_some() {
            selection_state.end_pos = Some(world_pos);
        }
    }

    // 3. Fin de la sélection (On relâche le clic)
    if buttons.just_released(MouseButton::Left) {
        if let (Some(start), Some(end)) = (selection_state.start_pos, selection_state.end_pos) {
            let min_x = start.x.min(end.x);
            let max_x = start.x.max(end.x);
            let min_y = start.y.min(end.y);
            let max_y = start.y.max(end.y);

            let is_click = (max_x - min_x) < 5.0 && (max_y - min_y) < 5.0;

            // Désélectionner tout
            for (entity, _, _, selected) in q_units.iter() {
                if selected.is_some() {
                    commands.entity(entity).remove::<Selected>();
                }
            }

            // Sélectionner les unités dans la zone
            for (entity, transform, collider, _) in q_units.iter() {
                let pos = transform.translation.truncate();
                let size = collider.0;

                if is_click {
                    let is_clicked = world_pos.x >= pos.x - size && world_pos.x <= pos.x + size &&
                                     world_pos.y >= pos.y - size && world_pos.y <= pos.y + size;
                    if is_clicked {
                        commands.entity(entity).insert(Selected);
                    }
                } else {
                    let in_box = pos.x >= min_x && pos.x <= max_x &&
                                 pos.y >= min_y && pos.y <= max_y;
                    if in_box {
                        commands.entity(entity).insert(Selected);
                    }
                }
            }
        }

        selection_state.start_pos = None;
        selection_state.end_pos = None;
    }
}

pub fn draw_selection_box(
    mut gizmos: Gizmos,
    selection_state: Res<SelectionState>,
) {
    if let (Some(start), Some(end)) = (selection_state.start_pos, selection_state.end_pos) {
        let size = (end - start).abs();
        if size.x > 5.0 || size.y > 5.0 {
            let center = (start + end) / 2.0;
            gizmos.rect_2d(center, size, Color::srgb(0.0, 1.0, 0.0));
        }
    }
}
