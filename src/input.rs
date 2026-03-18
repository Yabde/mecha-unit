use bevy::prelude::*;
use crate::units::Selected;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_selection);
    }
}

fn handle_selection(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    // On récupère l'Entité (son ID), sa position, sa taille, et on regarde si elle est déjà sélectionnée
    q_units: Query<(Entity, &Transform, &Sprite, Option<&Selected>)>, 
) {
    // Si on vient de faire un clic gauche...
    if buttons.just_pressed(MouseButton::Left) {
        // 1. Récupérer la fenêtre, la caméra, et la position de la souris
        let Some(cursor_position) = window.cursor_position() else { return; };
        let (camera, camera_transform) = *camera_query;
        
        // 2. Transformer la position de l'écran en position dans le monde du jeu
        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

        // 3. Vérifier si on a cliqué sur une unité
        for (entity, transform, sprite, selected) in q_units.iter() {
            let size = sprite.custom_size.unwrap_or(Vec2::splat(40.0)) / 2.0;
            let pos = transform.translation.truncate(); // Retire l'axe Z
            
            // Calcul d'une "Bounding Box" (Boîte de collision) très basique
            let is_clicked = world_position.x >= pos.x - size.x && world_position.x <= pos.x + size.x &&
                             world_position.y >= pos.y - size.y && world_position.y <= pos.y + size.y;
                             
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