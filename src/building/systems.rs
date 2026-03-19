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
    mut q_ghost: Query<(Entity, &mut Transform, &MeshMaterial2d<ColorMaterial>), With<GhostBuilding>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_interactions: Query<&Interaction, With<Node>>, // NEW: Détecteur d'UI
    q_colliders: Query<(&Transform, &crate::units::components::PhysicalCollider), Without<GhostBuilding>>, // Collision Check
) {
    let active_building = match placement_state.active_building {
        Some(b) => b,
        None => {
            for (entity, _, _) in q_ghost.iter() {
                commands.entity(entity).despawn();
            }
            return;
        }
    };

    // Empêche l'action si la souris survole N'IMPORTE QUEL élément UI
    for interaction in q_interactions.iter() {
        if *interaction != Interaction::None { return; }
    }

    let Some(cursor_position) = window.cursor_position() else { return; };
    
    let minimap_rect = Rect::from_corners(
        Vec2::new(window.width() - 318.0, window.height() - 318.0),
        Vec2::new(window.width(), window.height())
    );
    if minimap_rect.contains(cursor_position) { return; }

    let (camera, camera_transform) = *camera_query;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    // Vérifier les collisions à world_pos
    let ghost_size = match active_building {
        BuildingType::Barracks => 30.0,
        BuildingType::Turret => 20.0,
    };

    let mut is_overlapping = false;
    for (t, collider) in q_colliders.iter() {
        let distance = world_pos.distance(t.translation.truncate());
        if distance < ghost_size + collider.0 {
            is_overlapping = true;
            break;
        }
    }

    // Gestion du Fantôme visuel
    if q_ghost.is_empty() {
        spawn_ghost_building(&mut commands, &mut meshes, &mut materials, active_building);
    } else {
        for (_, mut transform, mat) in q_ghost.iter_mut() {
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
            
            // Changer la couleur du material !
            if let Some(mat_asset) = materials.get_mut(mat.id()) {
                mat_asset.color = if is_overlapping {
                    Color::srgba(1.0, 0.0, 0.0, 0.5) // Rouge = Interdit
                } else {
                    Color::srgba(0.0, 1.0, 0.0, 0.5) // Vert = Autorisé
                };
            }
        }
    }

    if buttons.just_pressed(MouseButton::Right) {
        placement_state.active_building = None;
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        if !is_overlapping && economy.crystals >= active_building.cost() {
            economy.crystals -= active_building.cost();
            spawn_construction(&mut commands, &mut meshes, &mut materials, active_building, world_pos, 1);
            placement_state.active_building = None; // Mode placement terminé
        }
    }
}
