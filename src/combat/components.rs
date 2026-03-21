use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Component, PartialEq, Eq)]
pub struct Team(pub u8);

#[derive(Component)]
pub struct MeleeRange(pub f32);

/// Texte flottant qui s'élève et disparaît pour signaler les dégâts
#[derive(Component)]
pub struct DamagePopup {
    pub lifetime: Timer,
}
