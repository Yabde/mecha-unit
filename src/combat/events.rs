use bevy::prelude::*;

#[derive(Message)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub base_damage: f32,
}
