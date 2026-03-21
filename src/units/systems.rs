use bevy::prelude::*;
use crate::units::components::*;

use crate::factory::units::{spawn_melee, spawn_worker};

pub fn setup_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Team 1 (Allies)
    spawn_melee(&mut commands, &mut meshes, &mut materials, UnitType::MeleeA, Vec2::new(-100.0, 0.0), 1);
    spawn_melee(&mut commands, &mut meshes, &mut materials, UnitType::MeleeB, Vec2::new(0.0, 0.0), 1);
    spawn_melee(&mut commands, &mut meshes, &mut materials, UnitType::MeleeC, Vec2::new(100.0, 0.0), 1);
    spawn_worker(&mut commands, &mut meshes, &mut materials, Vec2::new(0.0, -150.0), 1);

    // Team 2 (Ennemis)
    spawn_melee(&mut commands, &mut meshes, &mut materials, UnitType::MeleeA, Vec2::new(250.0, 0.0), 2);
    spawn_melee(&mut commands, &mut meshes, &mut materials, UnitType::MeleeB, Vec2::new(250.0, 0.0), 2);
    spawn_melee(&mut commands, &mut meshes, &mut materials, UnitType::MeleeC, Vec2::new(250.0, 0.0), 2);
}

pub fn animate_selection(mut query: Query<(&mut Transform, Option<&Selected>)>) {
    for (mut transform, selected) in query.iter_mut() {
        if selected.is_some() {
            transform.scale = Vec3::splat(1.3);
        } else {
            transform.scale = Vec3::splat(1.0);
        }
    }
}

pub fn move_units(
    mut commands: Commands,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &mut Transform, &TargetPosition, &Speed, Option<&Selected>)>,
) {
    for (entity, mut transform, target, speed, selected) in query.iter_mut() {
        let current_pos = transform.translation.truncate();
        let direction = target.0 - current_pos;
        let distance = direction.length();

        if selected.is_some() {
            gizmos.line_2d(current_pos, target.0, Color::WHITE);
            gizmos.circle_2d(target.0, 5.0, Color::srgb(1.0, 1.0, 0.0));
        }

        if distance < 2.0 {
            transform.translation.x = target.0.x;
            transform.translation.y = target.0.y;
            commands.entity(entity).remove::<TargetPosition>();
        } else {
            let movement = direction.normalize() * speed.0 * time.delta_secs();
            transform.translation += movement.extend(0.0);
        }
    }
}

pub fn handle_separation(
    mut query: Query<(&mut Transform, &PhysicalCollider, Option<&Speed>)>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut transform_a, collider_a, speed_a), (mut transform_b, collider_b, speed_b)]) = combinations.fetch_next() {
        let pos_a = transform_a.translation.truncate();
        let pos_b = transform_b.translation.truncate();
        
        let mut diff = pos_a - pos_b;
        if diff.length_squared() < 0.001 {
            diff = Vec2::new(1.0, 0.0);
        }
        
        let distance = diff.length();
        let min_distance = collider_a.0 + collider_b.0;

        if distance < min_distance {
            let overlap = min_distance - distance;
            let direction = diff.normalize();
            let push_force = direction * overlap * 15.0 * time.delta_secs();

            if speed_a.is_some() && speed_b.is_some() {
                // Les deux sont mobiles (Unités), on les écarte mutuellement
                transform_a.translation += push_force.extend(0.0);
                transform_b.translation -= push_force.extend(0.0);
            } else if speed_a.is_some() {
                // Seul A est mobile, il "glisse" le long de B (Bâtiment)
                transform_a.translation += (push_force * 2.0).extend(0.0);
            } else if speed_b.is_some() {
                // Seul B est mobile
                transform_b.translation -= (push_force * 2.0).extend(0.0);
            }
        }
    }
}
