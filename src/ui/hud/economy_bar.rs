use bevy::prelude::*;
use crate::economy::resources::PlayerEconomy;
use crate::ui::theme;
use super::components::EconomyText;

pub fn setup_economy_bar(mut commands: Commands) {
    commands.spawn((
        Text::new("Cristaux: 0"),
        TextFont {
            font_size: theme::TEXT_SIZE_HUD,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        },
        EconomyText,
    ));
}

pub fn update_economy_bar(
    economy: Res<PlayerEconomy>,
    mut q_text: Query<&mut Text, With<EconomyText>>,
) {
    if economy.is_changed() {
        for mut text in q_text.iter_mut() {
            text.0 = format!("Cristaux: {}", economy.crystals);
        }
    }
}
