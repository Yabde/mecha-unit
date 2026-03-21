use bevy::prelude::*;
use crate::economy::resources::PlayerEconomy;
use crate::ui::theme;
use super::components::*;

/// Crée la barre de dashboard en haut de l'écran (ressources + armée)
pub fn setup_top_bar(mut commands: Commands) {
    // Conteneur horizontal top bar
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(theme::PANEL_BG),
    )).with_children(|parent| {
        // Gauche : Cristaux
        parent.spawn((
            Text::new("[Cristaux] 0"),
            TextFont { font_size: theme::TEXT_SIZE_HUD, ..default() },
            TextColor(Color::WHITE),
            EconomyText,
        ));

        // Droite : Compteur d'armée 
        parent.spawn((
            Text::new("[Armee] W:0 | A:0 | B:0 | C:0"),
            TextFont { font_size: theme::TEXT_SIZE_HUD, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ArmyCountText,
        ));
    });
}

pub fn update_economy_bar(
    economy: Res<PlayerEconomy>,
    mut q_text: Query<&mut Text, With<EconomyText>>,
) {
    if economy.is_changed() {
        for mut text in q_text.iter_mut() {
            text.0 = format!("[Cristaux] {}", economy.crystals);
        }
    }
}
