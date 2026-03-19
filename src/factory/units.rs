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
        Transform::from_xyz(position.x, position.y, 0.0),
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
    )).insert(MineTimer(Timer::from_seconds(0.5, TimerMode::Repeating))).id()
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

    commands.spawn((
        unit_type,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(position.x, position.y, 0.0),
        Speed(speed),
        SelectionCollider(20.0),
        PhysicalCollider(18.0),
        Team(team),
        Health(health),
        Damage(damage),
        MeleeRange(45.0),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once)),
    )).id()
}
