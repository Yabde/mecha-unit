use bevy::prelude::*;
use crate::input::helpers::{is_over_minimap, is_ui_blocking};
use crate::units::components::{Selected, TargetPosition, PhysicalCollider};
use crate::economy::components::{ResourceNode, WorkerState};
use crate::building::resources::PlacementState;

pub fn handle_movement_orders(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    q_selected_units: Query<Entity, With<Selected>>,
    q_resources: Query<(Entity, &Transform, &PhysicalCollider), With<ResourceNode>>,
    mut q_worker_states: Query<&mut WorkerState>,
    placement_state: Res<PlacementState>,
    q_interactions: Query<&Interaction, With<Node>>,
) {
    if placement_state.active_building.is_some() { return; }
    if is_ui_blocking(&q_interactions) { return; }

    if buttons.just_pressed(MouseButton::Right) {
        let Some(cursor_position) = window.cursor_position() else { return; };
        if is_over_minimap(&window, cursor_position) { return; }

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
                    *state = WorkerState::MovingToResource(res_entity);
                    commands.entity(entity).remove::<TargetPosition>();
                } else {
                    // Les unités de combat vont juste se déplacer à côté
                    commands.entity(entity).insert(TargetPosition(world_position));
                }
            } else {
                // Clic sur le sol classique
                commands.entity(entity).insert(TargetPosition(world_position));

                // Si c'est un ouvrier, on annule son travail en cours
                if let Ok(mut state) = q_worker_states.get_mut(entity) {
                    *state = WorkerState::Idle;
                }
            }
        }
    }
}
