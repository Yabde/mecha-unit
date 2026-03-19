use bevy::prelude::*;
use crate::building::components::{BuildingType, GhostBuilding};
use crate::building::resources::PlacementState;
use crate::economy::resources::PlayerEconomy;
use crate::factory::buildings::{spawn_construction, spawn_ghost_building};

pub fn handle_placement(
    mut commands: Commands,
    mut placement_state: ResMut<PlacementState>,
    mut economy: ResMut<PlayerEconomy>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mut q_ghost: Query<(Entity, &mut Transform), With<GhostBuilding>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let active_building = match placement_state.active_building {
        Some(b) => b,
        None => {
            // S'il n'y a pas de construction active, on détruit le fantôme
            for (entity, _) in q_ghost.iter() {
                commands.entity(entity).despawn();
            }
            return;
        }
    };

    let Some(cursor_position) = window.cursor_position() else { return; };
    
    // Zone occupée par la minimap
    let minimap_rect = Rect::from_corners(
        Vec2::new(window.width() - 318.0, window.height() - 318.0),
        Vec2::new(window.width(), window.height())
    );
    if minimap_rect.contains(cursor_position) { return; }

    let (camera, camera_transform) = *camera_query;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    // Gestion du Fantôme visuel
    if q_ghost.is_empty() {
        spawn_ghost_building(&mut commands, &mut meshes, &mut materials, active_building);
    } else {
        for (_, mut transform) in q_ghost.iter_mut() {
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
        }
    }

    // Annuler le placement (Clic Droit)
    if buttons.just_pressed(MouseButton::Right) {
        placement_state.active_building = None;
        return;
    }

    // Confirmer le placement (Clic Gauche)
    if buttons.just_pressed(MouseButton::Left) {
        if economy.crystals >= active_building.cost() {
            economy.crystals -= active_building.cost();
            spawn_construction(&mut commands, &mut meshes, &mut materials, active_building, world_pos, 1);
            placement_state.active_building = None; // Mode placement terminé
        }
    }
}
