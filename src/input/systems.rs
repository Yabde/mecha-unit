use bevy::prelude::*;
use crate::input::resources::SelectionState;
use crate::units::components::{Selected, SelectionCollider};

pub fn handle_selection_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    // On sécurise la requête en ciblant uniquement la caméra principale (pas la minimap)
    camera_query: Single<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mut selection_state: ResMut<SelectionState>,
    q_units: Query<(Entity, &Transform, &SelectionCollider, Option<&Selected>)>, 
    placement_state: Res<crate::building::resources::PlacementState>,
) {
    if placement_state.active_building.is_some() { return; }
    
    let Some(cursor_position) = window.cursor_position() else { return; };
    let (camera, camera_transform) = *camera_query;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    // Zone occupée par la minimap (308x308 pixels, placée à 10px du bord bas-droit)
    let minimap_rect = Rect::from_corners(
        Vec2::new(window.width() - 318.0, window.height() - 318.0),
        Vec2::new(window.width(), window.height())
    );
    let is_over_minimap = minimap_rect.contains(cursor_position);

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

            for (entity, _, _, selected) in q_units.iter() {
                if selected.is_some() {
                    commands.entity(entity).remove::<Selected>();
                }
            }

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

pub fn handle_movement_orders(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    q_selected_units: Query<Entity, With<crate::units::components::Selected>>,
    q_resources: Query<(Entity, &Transform, &crate::units::components::PhysicalCollider), With<crate::economy::components::ResourceNode>>,
    mut q_worker_states: Query<&mut crate::economy::components::WorkerState>,
    placement_state: Res<crate::building::resources::PlacementState>,
) {
    if placement_state.active_building.is_some() { return; }

    if buttons.just_pressed(MouseButton::Right) {
        let Some(cursor_position) = window.cursor_position() else { return; };
        
        let minimap_rect = Rect::from_corners(
            Vec2::new(window.width() - 318.0, window.height() - 318.0),
            Vec2::new(window.width(), window.height())
        );
        if minimap_rect.contains(cursor_position) { return; } // On ignore si on clique sur la minimap

        let (camera, camera_transform) = *camera_query;
        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

        // Vérification du clic sur une ressource
        let mut clicked_resource = None;
        for (res_entity, res_transform, res_collider) in q_resources.iter() {
            let pos = res_transform.translation.truncate();
            if pos.distance(world_position) <= res_collider.0 {
                clicked_resource = Some(res_entity);
                break;
            }
        }

        for entity in q_selected_units.iter() {
            if let Some(res_entity) = clicked_resource {
                // S'il s'agit d'un ouvrier, on lance l'ordre de minage
                if let Ok(mut state) = q_worker_states.get_mut(entity) {
                    *state = crate::economy::components::WorkerState::MovingToResource(res_entity);
                    // On supprime un éventuel mouvement de déplacement pour qu'il s'arrête et lance son IA minage
                    commands.entity(entity).remove::<crate::units::components::TargetPosition>();
                } else {
                    // Les unités de combat vont juste se déplacer à côté
                    commands.entity(entity).insert(crate::units::components::TargetPosition(world_position));
                }
            } else {
                // Clic sur le sol classique ou sur l'ennemi
                commands.entity(entity).insert(crate::units::components::TargetPosition(world_position));
                
                // Si c'est un ouvrier, on annule son travail en cours pour obéir à l'ordre direct
                if let Ok(mut state) = q_worker_states.get_mut(entity) {
                    *state = crate::economy::components::WorkerState::Idle;
                }
            }
        }
    }
}
