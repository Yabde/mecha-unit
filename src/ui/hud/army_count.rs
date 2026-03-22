use bevy::prelude::*;
use crate::units::components::UnitType;
use crate::combat::components::Team;
use super::components::ArmyCountText;

pub fn update_army_count(
    q_units: Query<(&UnitType, &Team)>,
    mut q_text: Query<&mut Text, With<ArmyCountText>>,
) {
    let mut workers = 0u32;
    let mut melee_a = 0u32;
    let mut melee_b = 0u32;
    let mut melee_c = 0u32;
    let mut ranged_a = 0u32;

    for (unit_type, team) in q_units.iter() {
        if team.0 != 1 { continue; }
        match unit_type {
            UnitType::Worker => workers += 1,
            UnitType::MeleeA => melee_a += 1,
            UnitType::MeleeB => melee_b += 1,
            UnitType::MeleeC => melee_c += 1,
            UnitType::RangedA => ranged_a += 1,
        }
    }

    for mut text in q_text.iter_mut() {
        text.0 = format!("[Armee] W:{} | A:{} | B:{} | C:{} | R:{}", workers, melee_a, melee_b, melee_c, ranged_a);
    }
}
