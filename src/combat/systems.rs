use bevy::prelude::*;
use crate::combat::components::*;
use crate::combat::events::*;
use crate::units::components::UnitType;

// 1. Détection des ennemis à portée
pub fn detect_combat(
    time: Res<Time>,
    mut attackers: Query<(Entity, &Transform, &Team, &Damage, &MeleeRange, &mut AttackTimer)>,
    defenders: Query<(Entity, &Transform, &Team), With<Health>>,
    mut attack_events: MessageWriter<AttackEvent>,
) {
    for (attacker_entity, attacker_transform, attacker_team, attacker_damage, range, mut timer) in attackers.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.fraction() < 1.0 { continue; }

        let attacker_pos = attacker_transform.translation.truncate();
        for (defender_entity, defender_transform, defender_team) in defenders.iter() {
            if attacker_team == defender_team { continue; }
            let def_pos = defender_transform.translation.truncate();
            if attacker_pos.distance(def_pos) <= range.0 {
                attack_events.write(AttackEvent {
                    attacker: attacker_entity,
                    defender: defender_entity,
                    base_damage: attacker_damage.0,
                });
                timer.0.reset();
                break;
            }
        }
    }
}

// 2. Traitement de l'attaque et calcul Pierre-Feuilles-Ciseaux
pub fn apply_damage(
    mut events: MessageReader<AttackEvent>,
    mut defenders: Query<(&mut Health, &UnitType)>,
    attackers: Query<&UnitType>,
) {
    for event in events.read() {
        let Ok((mut health, def_type)) = defenders.get_mut(event.defender) else { continue; };
        let Ok(att_type) = attackers.get(event.attacker) else { continue; };

        let multiplier = match (att_type, def_type) {
            (UnitType::TypeA, UnitType::TypeB) => 2.0,
            (UnitType::TypeB, UnitType::TypeC) => 2.0,
            (UnitType::TypeC, UnitType::TypeA) => 2.0,
            (UnitType::TypeA, UnitType::TypeC) => 0.5,
            (UnitType::TypeC, UnitType::TypeB) => 0.5,
            (UnitType::TypeB, UnitType::TypeA) => 0.5,
            _ => 1.0,
        };

        let final_damage = event.base_damage * multiplier;
        health.0 -= final_damage;
        println!("Frappe ! {:?} inflige {} dégâts à {:?} (Il reste {} PV)", att_type, final_damage, def_type, health.0);
    }
}

// 3. Mort : Despawn des entités dont la vie est à 0 ou moins
pub fn handle_death(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
            println!("💥 Unité détruite !");
        }
    }
}
