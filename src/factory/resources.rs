use bevy::prelude::*;
use crate::economy::components::ResourceNode;
use crate::units::components::{PhysicalCollider, SelectionCollider};

pub fn spawn_crystal(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    amount: f32,
) -> Entity {
    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(15.0, 6))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.8, 0.0))),
        Transform::from_xyz(position.x, position.y, 0.0),
        ResourceNode { amount },
        PhysicalCollider(15.0),
        SelectionCollider(20.0),
    )).id()
}
