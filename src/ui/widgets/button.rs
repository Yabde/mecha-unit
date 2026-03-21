use bevy::prelude::*;
use crate::ui::theme;

/// Fonction générique pour spawner un bouton d'action dans un panneau parent.
/// `marker` est le composant spécifique à attacher (BuildButton, ProductionButton, etc.)
pub fn spawn_action_button(
    commands: &mut Commands,
    parent_entity: Entity,
    text: &str,
    marker: impl Component,
) {
    let btn = commands.spawn((
        Button,
        Node {
            padding: UiRect::all(theme::BUTTON_PADDING),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(theme::BUTTON_BORDER),
            ..default()
        },
        BackgroundColor(theme::BUTTON_BG),
        marker,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(text),
            TextFont { font_size: theme::TEXT_SIZE_BUTTON, ..default() },
            TextColor(Color::WHITE),
        ));
    }).id();
    commands.entity(parent_entity).add_child(btn);
}
