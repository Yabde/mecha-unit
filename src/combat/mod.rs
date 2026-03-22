pub mod components;
pub mod events;
pub mod systems;
pub mod ranged;

use bevy::prelude::*;
use events::AttackEvent;
use systems::{detect_combat, apply_damage, handle_death, animate_damage_popups};
use ranged::{detect_ranged_combat, move_projectiles, apply_projectile_hits};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AttackEvent>()
           .add_systems(Update, (
               detect_combat, 
               detect_ranged_combat,
               apply_damage, 
               move_projectiles,
               apply_projectile_hits,
               animate_damage_popups, 
               handle_death,
           ).chain());
    }
}
