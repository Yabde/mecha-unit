use bevy::prelude::*;
use crate::units::{Selected, TargetPosition, SelectionCollider};

// On crée une ressource pour stocker l'état de notre boîte de sélection
#[derive(Resource, Default)]
pub struct SelectionState {
    pub start_pos: Option<Vec2>,
    pub end_pos: Option<Vec2>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        // Initialisation de la ressource state
        app.init_resource::<SelectionState>();
        
        // Ajout des systèmes
        app.add_systems(Update, (
            handle_selection_input,
            draw_selection_box, 
            handle_movement_orders
        ));
    }
}

// Gère la logique de la souris (clic, glisser, relâcher)
fn handle_selection_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut selection_state: ResMut<SelectionState>,
    q_units: Query<(Entity, &Transform, &SelectionCollider, Option<&Selected>)>, 
) {
    let Some(cursor_position) = window.cursor_position() else { return; };
    let (camera, camera_transform) = *camera_query;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    // 1. Début de la sélection (On clique)
    if buttons.just_pressed(MouseButton::Left) {
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
            // On calcule les limites de la boîte
            let min_x = start.x.min(end.x);
            let max_x = start.x.max(end.x);
            let min_y = start.y.min(end.y);
            let max_y = start.y.max(end.y);
            
            // Si la boîte est très petite, on considère que c'est un simple clic
            let is_click = (max_x - min_x) < 5.0 && (max_y - min_y) < 5.0;

            // Règle classique de RTS : on désélectionne tout d'abord (à moins d'utiliser Shift par exemple, on l'ajoutera plus tard si besoin)
            for (entity, _, _, selected) in q_units.iter() {
                if selected.is_some() {
                    commands.entity(entity).remove::<Selected>();
                }
            }

            // On sélectionne les unités qui se trouvent dans la zone
            for (entity, transform, collider, _) in q_units.iter() {
                let pos = transform.translation.truncate();
                let size = collider.0;
                
                if is_click {
                    // Logique de clic simple : L'unité est-elle sous la souris ?
                    let is_clicked = world_pos.x >= pos.x - size && world_pos.x <= pos.x + size &&
                                     world_pos.y >= pos.y - size && world_pos.y <= pos.y + size;
                    if is_clicked {
                        commands.entity(entity).insert(Selected);
                    }
                } else {
                    // Logique drag box : Le centre de l'unité est-il dans le rectangle ?
                    let in_box = pos.x >= min_x && pos.x <= max_x &&
                                 pos.y >= min_y && pos.y <= max_y;
                    if in_box {
                        commands.entity(entity).insert(Selected);
                    }
                }
            }
        }
        
        // On réinitialise notre boîte
        selection_state.start_pos = None;
        selection_state.end_pos = None;
    }
}

// Dessine un rectangle vert pour visualiser la boîte de sélection en cours
fn draw_selection_box(
    mut gizmos: Gizmos,
    selection_state: Res<SelectionState>,
) {
    if let (Some(start), Some(end)) = (selection_state.start_pos, selection_state.end_pos) {
        let size = (end - start).abs();
        // Ne dessine que si on a "glissé" un minimum
        if size.x > 5.0 || size.y > 5.0 {
            let center = (start + end) / 2.0;
            gizmos.rect_2d(center, size, Color::srgb(0.0, 1.0, 0.0));
        }
    }
}

fn handle_movement_orders(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    // Optimisation : On ne requête que les entités qui ont le marqueur "Selected"
    q_selected_units: Query<Entity, With<Selected>>, 
) {
    // Si on fait un clic droit, c'est un ordre de déplacement !
    if buttons.just_pressed(MouseButton::Right) {
        let Some(cursor_position) = window.cursor_position() else { return; };
        let (camera, camera_transform) = *camera_query;
        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

        // On assigne la position cliquée comme cible à toutes les unités sélectionnées
        for entity in q_selected_units.iter() {
            commands.entity(entity).insert(TargetPosition(world_position));
        }
    }
}