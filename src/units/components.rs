use bevy::prelude::*;

// Re-export des composants partagés pour compatibilité
pub use crate::core::components::*;

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
