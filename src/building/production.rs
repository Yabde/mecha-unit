use bevy::prelude::*;
use crate::building::components::ProductionQueue;
use crate::factory::units::{spawn_worker, spawn_melee, spawn_ranged};
use crate::units::components::UnitType;
use crate::combat::components::Team;

pub fn process_production_queues(
    mut commands: Commands,
    time: Res<Time>,
    mut q_buildings: Query<(&mut ProductionQueue, &Transform, &Team)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut production, transform, team) in q_buildings.iter_mut() {
        if production.queue.is_empty() {
            continue;
        }

        production.timer.tick(time.delta());

        if production.timer.just_finished() {
            let unit_to_spawn = production.queue.remove(0); // Dépile la première unité
            let pos = transform.translation.truncate(); // Apparaît au centre du bâtiment (éjectée par la collision)
            let team_id = team.0;

            match unit_to_spawn {
                UnitType::Worker => { 
                    spawn_worker(&mut commands, &mut meshes, &mut materials, pos, team_id); 
                },
                UnitType::RangedA => {
                    spawn_ranged(&mut commands, &mut meshes, &mut materials, pos, team_id);
                },
                _ => { 
                    spawn_melee(&mut commands, &mut meshes, &mut materials, unit_to_spawn, pos, team_id); 
                },
            }

            // Si d'autres unités patientent, on configure le timer pour la suivante !
            if let Some(next_unit) = production.queue.first() {
                production.timer = Timer::from_seconds(next_unit.build_time(), TimerMode::Once);
            }
        }
    }
}

pub fn draw_production_progress(
    q_buildings: Query<(&ProductionQueue, &Transform)>,
    mut gizmos: Gizmos,
) {
    for (production, transform) in q_buildings.iter() {
        if production.queue.is_empty() {
            continue; // Rien à afficher si aucune production
        }

        let ratio = production.timer.elapsed_secs() / production.timer.duration().as_secs_f32();
        
        let pos = transform.translation.truncate() + Vec2::new(0.0, 45.0); // 45px au dessus du centre
        let bar_width = 40.0;
        let bar_height = 6.0;

        // Fond Noir/Gris
        let half_size = Vec2::new(bar_width, bar_height) / 2.0;
        gizmos.rect_2d(pos, half_size, Color::srgba(0.1, 0.1, 0.1, 0.8));

        // Remplissage Vert 
        let fill_width = bar_width * ratio;
        let fill_pos = pos - Vec2::new((bar_width - fill_width) / 2.0, 0.0);
        let half_fill_size = Vec2::new(fill_width, bar_height) / 2.0;
        gizmos.rect_2d(fill_pos, half_fill_size, Color::srgb(0.0, 0.8, 0.0));
    }
}
