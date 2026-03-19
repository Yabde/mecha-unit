use bevy::prelude::*;
use crate::units::components::*;
use crate::combat::components::{Health, Damage, AttackTimer, Team, MeleeRange};
use crate::economy::components::{Worker, WorkerState, MineTimer};

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
        UnitType::MeleeA,
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
        UnitType::MeleeB,
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
        UnitType::MeleeC,
        Speed(150.0),
        SelectionCollider(20.0),
        PhysicalCollider(18.0),
        Health(50.0),
        Damage(10.0),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        Team(1),
        MeleeRange(45.0),
    ));

    // ---- Worker (Allié) ----
    commands.spawn((
        UnitType::Worker,
        Mesh2d(meshes.add(Circle::new(10.0))), // Petit cercle
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))), // Blanc
        Transform::from_xyz(0.0, -150.0, 0.0),
        Speed(140.0), // Plus rapide que la normale
        SelectionCollider(15.0),
        PhysicalCollider(10.0),
        Team(1),
        Health(50.0),
        Damage(0.0),
        MeleeRange(0.0),
        AttackTimer(Timer::from_seconds(999.0, TimerMode::Once)), // N'attaque pas
        Worker { capacity: 10.0, current_load: 0.0 },
        WorkerState::Idle,
    )).insert(MineTimer(Timer::from_seconds(0.5, TimerMode::Repeating))); // Vitesse de minage

    // --- ENNEMIS (Team 2) ---
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 20.0),
            Vec2::new(-20.0, -20.0),
            Vec2::new(20.0, -20.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(250.0, 0.0, 0.0),
        UnitType::MeleeC,
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
