use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType { TypeA, TypeB, TypeC }

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct TargetPosition(pub Vec2);

#[derive(Component)]
pub struct SelectionCollider(pub f32);

#[derive(Component)]
pub struct PhysicalCollider(pub f32);
