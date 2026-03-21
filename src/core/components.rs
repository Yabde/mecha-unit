use bevy::prelude::*;

/// Marqueur de sélection, applicable à toute entité (unité, bâtiment, ressource)
#[derive(Component)]
pub struct Selected;

/// Vitesse de déplacement, applicable à toute entité mobile
#[derive(Component)]
pub struct Speed(pub f32);

/// Position cible pour le déplacement
#[derive(Component)]
pub struct TargetPosition(pub Vec2);

/// Rayon de collision physique (pour la séparation entre entités)
#[derive(Component)]
pub struct PhysicalCollider(pub f32);

/// Rayon de sélection au clic (généralement un peu plus grand que PhysicalCollider)
#[derive(Component)]
pub struct SelectionCollider(pub f32);
