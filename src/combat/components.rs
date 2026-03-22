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

/// Composant d'attaque à distance (tourelles, unités ranged)
#[derive(Component)]
pub struct RangedAttack {
    pub range: f32,
    pub projectile_speed: f32,
}

/// Projectile en vol vers une cible
#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub damage: f32,
    pub speed: f32,
    pub origin: Vec2, // Position de depart (pour le trait visuel)
}
