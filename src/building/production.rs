use bevy::prelude::*;
use crate::building::components::ProductionQueue;
use crate::factory::units::{spawn_worker, spawn_melee};
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
