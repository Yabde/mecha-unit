pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use events::AttackEvent;
use systems::{detect_combat, apply_damage, handle_death};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AttackEvent>()
           .add_systems(Update, (detect_combat, apply_damage, handle_death).chain());
    }
}
