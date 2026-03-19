use bevy::prelude::*;
use crate::economy::components::Base;
use crate::units::components::{SelectionCollider, PhysicalCollider};
use crate::combat::components::{Team, Health};

pub fn spawn_base(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    team: u8,
) -> Entity {
    let color = if team == 1 { Color::srgb(0.0, 0.5, 0.8) } else { Color::srgb(0.5, 0.0, 0.0) };
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(position.x, position.y, 0.0),
        Base { team },
        PhysicalCollider(40.0),
        SelectionCollider(45.0),
        Team(team),
        Health(500.0),
    )).id()
}

use crate::building::components::BuildingType;
use crate::combat::components::Damage;

pub fn spawn_construction(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    building_type: BuildingType,
    position: Vec2,
    team: u8,
) -> Entity {
    let (mesh, color, size, health) = match building_type {
        BuildingType::Barracks => (
            meshes.add(Rectangle::new(60.0, 60.0)),
            if team == 1 { Color::srgb(0.0, 0.4, 0.0) } else { Color::srgb(0.4, 0.0, 0.0) },
            30.0, 300.0
        ),
        BuildingType::Turret => (
            meshes.add(Circle::new(20.0)),
            if team == 1 { Color::srgb(0.5, 0.5, 0.5) } else { Color::srgb(0.3, 0.3, 0.3) },
            20.0, 150.0
        ),
    };

    let mut entity = commands.spawn((
        building_type, // <--- L'oubli était ici ! On attache l'enum comme composant !
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(position.x, position.y, 0.0),
        PhysicalCollider(size),
        SelectionCollider(size + 5.0),
        Team(team),
        Health(health),
    ));

    if building_type == BuildingType::Turret {
        entity.insert(Damage(15.0));
    } else if building_type == BuildingType::Barracks {
        entity.insert(crate::building::components::ProductionQueue {
            queue: Vec::new(),
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        });
    }

    entity.id()
}

pub fn spawn_ghost_building(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    building_type: BuildingType,
) -> Entity {
    let mesh = match building_type {
        BuildingType::Barracks => meshes.add(Rectangle::new(60.0, 60.0)),
        BuildingType::Turret => meshes.add(Circle::new(20.0)),
    };

    commands.spawn((
        crate::building::components::GhostBuilding,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 1.0, 0.0, 0.5))),
        Transform::from_xyz(0.0, 0.0, 2.0),
    )).id()
}
