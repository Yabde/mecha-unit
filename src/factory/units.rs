use bevy::prelude::*;
use crate::units::components::*;
use crate::combat::components::{Health, Damage, AttackTimer, Team, MeleeRange};
use crate::economy::components::{Worker, WorkerState, MineTimer};

pub fn spawn_worker(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    team: u8,
) -> Entity {
    let color = if team == 1 { Color::srgb(1.0, 1.0, 1.0) } else { Color::srgb(0.5, 0.5, 0.5) };
    
    commands.spawn((
        UnitType::Worker,
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(position.x, position.y, 1.0),
        Speed(140.0),
        SelectionCollider(15.0),
        PhysicalCollider(10.0),
        Team(team),
        Health(50.0),
        Damage(0.0),
        MeleeRange(0.0),
        AttackTimer(Timer::from_seconds(999.0, TimerMode::Once)),
        Worker { capacity: 10.0, current_load: 0.0 },
        WorkerState::Idle,
    ))
    .with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(12.0))), // +2 pixels for black border
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.0, 0.0, -0.1), // Draw slightly behind
        ));
    })
    .insert(MineTimer(Timer::from_seconds(0.5, TimerMode::Repeating))).id()
}

pub fn spawn_melee(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    unit_type: UnitType,
    position: Vec2,
    team: u8,
) -> Entity {
    let (mesh, color, health, damage, speed) = match unit_type {
        UnitType::MeleeA => (
            meshes.add(Rectangle::new(40.0, 40.0)),
            if team == 1 { Color::srgb(1.0, 0.0, 0.0) } else { Color::srgb(0.4, 0.0, 0.0) },
            50.0, 10.0, 150.0
        ),
        UnitType::MeleeB => (
            meshes.add(Circle::new(20.0)),
            if team == 1 { Color::srgb(0.0, 1.0, 0.0) } else { Color::srgb(0.0, 0.4, 0.0) },
            50.0, 10.0, 150.0
        ),
        UnitType::MeleeC | _ => (
            meshes.add(Triangle2d::new(Vec2::new(0.0, 20.0), Vec2::new(-20.0, -20.0), Vec2::new(20.0, -20.0))),
            if team == 1 { Color::srgb(0.0, 0.0, 1.0) } else { Color::srgb(0.2, 0.2, 0.2) }, // Gris foncé pour les méchants par défaut
            100.0, 10.0, 150.0
        ),
    };

    let border_mesh = match unit_type {
        UnitType::MeleeA => meshes.add(Rectangle::new(44.0, 44.0)),
        UnitType::MeleeB => meshes.add(Circle::new(22.0)),
        UnitType::MeleeC | _ => meshes.add(Triangle2d::new(Vec2::new(0.0, 22.0), Vec2::new(-22.0, -22.0), Vec2::new(22.0, -22.0))),
    };

    commands.spawn((
        unit_type,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(position.x, position.y, 1.0),
        Speed(speed),
        SelectionCollider(20.0),
        PhysicalCollider(18.0),
        Team(team),
        Health(health),
        Damage(damage),
        MeleeRange(45.0),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Mesh2d(border_mesh),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.0, 0.0, -0.1),
        ));
    }).id()
}

pub fn spawn_ranged(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    team: u8,
) -> Entity {
    let color = if team == 1 { Color::srgb(0.9, 0.6, 0.0) } else { Color::srgb(0.4, 0.3, 0.0) }; // Orange/ambré

    // Losange (diamond) : un carre tourne a 45 degrés
    let mesh = meshes.add(Rectangle::new(28.0, 28.0));
    let border_mesh = meshes.add(Rectangle::new(32.0, 32.0));

    commands.spawn((
        UnitType::RangedA,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(position.x, position.y, 1.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)), // 45 degrés = losange !
        Speed(120.0), // Plus lent que les melee
        SelectionCollider(20.0),
        PhysicalCollider(14.0),
        Team(team),
        Health(80.0), // Fragile
        Damage(12.0),
        AttackTimer(Timer::from_seconds(1.2, TimerMode::Once)),
        crate::combat::components::RangedAttack {
            range: 150.0,
            projectile_speed: 300.0,
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Mesh2d(border_mesh),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.0, 0.0, -0.1),
        ));
    }).id()
}
