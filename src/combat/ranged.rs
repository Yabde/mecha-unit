use bevy::prelude::*;
use crate::combat::components::*;

/// Detection des cibles pour les entites disposant de RangedAttack
pub fn detect_ranged_combat(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut attackers: Query<(&Transform, &Team, &Damage, &RangedAttack, &mut AttackTimer)>,
    defenders: Query<(Entity, &Transform, &Team), With<Health>>,
) {
    for (attacker_transform, attacker_team, attacker_damage, ranged, mut timer) in attackers.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.fraction() < 1.0 { continue; }

        let attacker_pos = attacker_transform.translation.truncate();
        for (defender_entity, defender_transform, defender_team) in defenders.iter() {
            if attacker_team == defender_team { continue; }
            let def_pos = defender_transform.translation.truncate();
            if attacker_pos.distance(def_pos) <= ranged.range {
                // Spawn un projectile !
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(4.0))),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))), // Jaune vif
                    Transform::from_xyz(attacker_pos.x, attacker_pos.y, 5.0),
                    Projectile {
                        target: defender_entity,
                        damage: attacker_damage.0,
                        speed: ranged.projectile_speed,
                        origin: attacker_pos,
                    },
                ));
                timer.0.reset();
                break;
            }
        }
    }
}

/// Deplace les projectiles vers leur cible et gere l'impact
pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut q_projectiles: Query<(Entity, &mut Transform, &Projectile)>,
    q_targets: Query<&Transform, (With<Health>, Without<Projectile>)>,
) {
    for (proj_entity, mut proj_transform, projectile) in q_projectiles.iter_mut() {
        // Cible encore vivante ?
        let Ok(target_transform) = q_targets.get(projectile.target) else {
            // Cible morte, le projectile se dissipe
            commands.entity(proj_entity).despawn();
            continue;
        };

        let proj_pos = proj_transform.translation.truncate();
        let target_pos = target_transform.translation.truncate();
        let direction = target_pos - proj_pos;
        let distance = direction.length();

        // Trait de trainee (Gizmo line)
        gizmos.line_2d(projectile.origin, proj_pos, Color::srgba(1.0, 0.8, 0.0, 0.3));

        // Impact !
        if distance < 8.0 {
            // Appliquer les degats directement
            commands.entity(proj_entity).despawn();

            // Spawn le popup de degats
            commands.spawn((
                Text2d::new(format!("-{}", projectile.damage as i32)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                Transform::from_xyz(target_pos.x, target_pos.y + 20.0, 10.0),
                DamagePopup {
                    lifetime: Timer::from_seconds(0.8, TimerMode::Once),
                },
            ));

            // On envoie l'evenement de degats via un mini-insert
            // On modifie la HP directement ici car on a deja le target
            // Note: on ne peut pas query mut Health ici (conflit), on utilise commands
            commands.entity(projectile.target).insert(ProjectileHit(projectile.damage));
        } else {
            // Avancer vers la cible
            let movement = direction.normalize() * projectile.speed * time.delta_secs();
            proj_transform.translation += movement.extend(0.0);
        }
    }
}

/// Marqueur temporaire pour appliquer les degats d'un projectile
#[derive(Component)]
pub struct ProjectileHit(pub f32);

/// Applique les degats des projectiles qui ont touche
pub fn apply_projectile_hits(
    mut commands: Commands,
    mut q_hit: Query<(Entity, &mut Health, &ProjectileHit)>,
) {
    for (entity, mut health, hit) in q_hit.iter_mut() {
        health.0 -= hit.0;
        commands.entity(entity).remove::<ProjectileHit>();
    }
}
