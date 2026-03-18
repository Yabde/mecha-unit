use bevy::prelude::*;
use crate::units::{Selected, TargetPosition, SelectionCollider};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_selection, handle_movement_orders));
    }
}

fn handle_selection(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    // On récupère l'Entité (son ID), sa position, sa taille via le collider, et on regarde si elle est déjà sélectionnée
    q_units: Query<(Entity, &Transform, &SelectionCollider, Option<&Selected>)>, 
) {
    // Si on vient de faire un clic gauche...
    if buttons.just_pressed(MouseButton::Left) {
        // 1. Récupérer la fenêtre, la caméra, et la position de la souris
        let Some(cursor_position) = window.cursor_position() else { return; };
        let (camera, camera_transform) = *camera_query;
        
        // 2. Transformer la position de l'écran en position dans le monde du jeu
        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

        // 3. Vérifier si on a cliqué sur une unité
        for (entity, transform, collider, selected) in q_units.iter() {
            let size = collider.0;
            let pos = transform.translation.truncate(); // Retire l'axe Z
            
            // Calcul d'une "Bounding Box" (Boîte de collision) très basique
            let is_clicked = world_position.x >= pos.x - size && world_position.x <= pos.x + size &&
                             world_position.y >= pos.y - size && world_position.y <= pos.y + size;
                             
            if is_clicked {
                if selected.is_none() {
                    commands.entity(entity).insert(Selected); // On attache le composant !
                    println!("Unité sélectionnée !");
                }
            } else {
                // Si on a cliqué ailleurs, on désélectionne
                if selected.is_some() {
                    commands.entity(entity).remove::<Selected>(); // On retire le composant
                }
            }
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