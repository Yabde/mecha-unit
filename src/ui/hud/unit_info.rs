use bevy::prelude::*;
use crate::units::components::UnitType;
use crate::core::components::{Selected, Speed};
use crate::combat::components::{Health, Damage};
use crate::ui::theme;
use super::components::*;

pub fn setup_unit_info_panel(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(60.0),
            left: Val::Px(15.0),
            display: Display::None, // Caché par défaut
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            padding: UiRect::all(Val::Px(12.0)),
            min_width: Val::Px(200.0),
            ..default()
        },
        BackgroundColor(theme::PANEL_BG),
        UnitInfoPanel,
    )).with_children(|parent| {
        // Titre
        parent.spawn((
            Text::new("-- Unite --"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.2)),
            UnitInfoType,
        ));
        // HP
        parent.spawn((
            Text::new("HP: --"),
            TextFont { font_size: theme::TEXT_SIZE_BUTTON, ..default() },
            TextColor(Color::srgb(0.3, 1.0, 0.3)),
            UnitInfoHP,
        ));
        // Dégâts
        parent.spawn((
            Text::new("ATK: --"),
            TextFont { font_size: theme::TEXT_SIZE_BUTTON, ..default() },
            TextColor(Color::srgb(1.0, 0.5, 0.5)),
            UnitInfoDamage,
        ));
        // Vitesse
        parent.spawn((
            Text::new("SPD: --"),
            TextFont { font_size: theme::TEXT_SIZE_BUTTON, ..default() },
            TextColor(Color::srgb(0.5, 0.7, 1.0)),
            UnitInfoSpeed,
        ));
    });
}

pub fn update_unit_info_panel(
    q_selected: Query<(&UnitType, &Health, Option<&Damage>, Option<&Speed>), With<Selected>>,
    mut q_panel: Query<&mut Node, With<UnitInfoPanel>>,
    mut q_type_text: Query<&mut Text, (With<UnitInfoType>, Without<UnitInfoHP>, Without<UnitInfoDamage>, Without<UnitInfoSpeed>)>,
    mut q_hp_text: Query<&mut Text, (With<UnitInfoHP>, Without<UnitInfoType>, Without<UnitInfoDamage>, Without<UnitInfoSpeed>)>,
    mut q_dmg_text: Query<&mut Text, (With<UnitInfoDamage>, Without<UnitInfoType>, Without<UnitInfoHP>, Without<UnitInfoSpeed>)>,
    mut q_spd_text: Query<&mut Text, (With<UnitInfoSpeed>, Without<UnitInfoType>, Without<UnitInfoHP>, Without<UnitInfoDamage>)>,
) {
    let selected_count = q_selected.iter().count();

    // Afficher uniquement si exactement 1 entité sélectionnée
    if let Some(mut node) = q_panel.iter_mut().next() {
        if selected_count == 1 {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
            return;
        }
    }

    // Mettre à jour les textes
    if let Some((unit_type, health, damage, speed)) = q_selected.iter().next() {
        let type_name = match unit_type {
            UnitType::Worker => "Ouvrier",
            UnitType::MeleeA => "MeleeA (Carre)",
            UnitType::MeleeB => "MeleeB (Cercle)",
            UnitType::MeleeC => "MeleeC (Triangle)",
            UnitType::RangedA => "RangedA (Losange)",
        };

        for mut text in q_type_text.iter_mut() {
            text.0 = format!("— {} —", type_name);
        }
        for mut text in q_hp_text.iter_mut() {
            text.0 = format!("HP: {:.0}", health.0);
        }
        for mut text in q_dmg_text.iter_mut() {
            text.0 = format!("ATK: {:.0}", damage.map_or(0.0, |d| d.0));
        }
        for mut text in q_spd_text.iter_mut() {
            text.0 = format!("SPD: {:.0}", speed.map_or(0.0, |s| s.0));
        }
    }
}
