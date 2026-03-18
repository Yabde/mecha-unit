use bevy::prelude::*;
use crate::units::components::*;
use crate::combat::components::{Health, Damage, AttackTimer, Team, MeleeRange};

pub fn setup_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Type A : Carré (Rouge)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        UnitType::TypeA,
        Speed(150.0),
        SelectionCollider(20.0),
        PhysicalCollider(18.0),
        Health(50.0),
        Damage(10.0),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        Team(1),
        MeleeRange(45.0),
    ));

    // Type B : Cercle (Vert)
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        UnitType::TypeB,
        Speed(150.0),
        SelectionCollider(20.0),
        PhysicalCollider(18.0),
        Health(50.0),
        Damage(10.0),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        Team(1),
        MeleeRange(45.0),
    ));

    // Type C : Triangle (Bleu)
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 20.0),
            Vec2::new(-20.0, -20.0),
            Vec2::new(20.0, -20.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(100.0, 0.0, 0.0),
        UnitType::TypeC,
        Speed(150.0),
        SelectionCollider(20.0),
        PhysicalCollider(18.0),
        Health(50.0),
        Damage(10.0),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        Team(1),
        MeleeRange(45.0),
    ));

    // --- ENNEMIS (Team 2) ---
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 20.0),
            Vec2::new(-20.0, -20.0),
            Vec2::new(20.0, -20.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(250.0, 0.0, 0.0),
        UnitType::TypeC,
        Speed(150.0),
        SelectionCollider(20.0),
        PhysicalCollider(18.0),
        Health(100.0),
        Damage(10.0),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        Team(2),
        MeleeRange(45.0),
    ));
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
    mut query: Query<(&mut Transform, &PhysicalCollider)>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut transform_a, collider_a), (mut transform_b, collider_b)]) = combinations.fetch_next() {
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

            transform_a.translation += push_force.extend(0.0);
            transform_b.translation -= push_force.extend(0.0);
        }
    }
}
