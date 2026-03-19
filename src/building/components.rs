use bevy::prelude::*;
use crate::units::components::UnitType;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    Barracks,
    Turret,
}

impl BuildingType {
    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::Barracks => 100,
            BuildingType::Turret => 50,
        }
    }
}

#[derive(Component)]
pub struct GhostBuilding;

#[derive(Component)]
pub struct ProductionQueue {
    pub queue: Vec<UnitType>,
    pub timer: Timer,
}
