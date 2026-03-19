use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    MeleeA,
    MeleeB,
    MeleeC,
    Worker,
}

impl UnitType {
    pub fn cost(&self) -> u32 {
        match self {
            UnitType::Worker => 50,
            UnitType::MeleeA | UnitType::MeleeB | UnitType::MeleeC => 100,
        }
    }

    pub fn build_time(&self) -> f32 {
        match self {
            UnitType::Worker => 2.0,
            UnitType::MeleeA | UnitType::MeleeB | UnitType::MeleeC => 4.0,
        }
    }
}

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
