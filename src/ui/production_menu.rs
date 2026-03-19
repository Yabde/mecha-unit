use bevy::prelude::*;
use crate::building::components::{BuildingType, ProductionQueue};
use crate::units::components::{UnitType, Selected};
use crate::economy::resources::PlayerEconomy;

#[derive(Component)]
pub struct ProductionMenuUI;

#[derive(Component)]
pub struct ProductionButton(pub UnitType);

pub fn setup_production_menu(mut commands: Commands) {
    let parent = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            right: Val::Px(350.0), // À gauche de la minimap
            display: Display::None,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        Interaction::None, // Bloque les clics pour la carte
        ProductionMenuUI,
    )).id();

    let mut spawn_btn = |text: &str, unit: UnitType| {
        let btn = commands.spawn((
            Button,
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            ProductionButton(unit),
        )).with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
        }).id();
        commands.entity(parent).add_child(btn);
    };

    spawn_btn("Worker (50)", UnitType::Worker);
    spawn_btn("Guerrier (100)", UnitType::MeleeA);
    // On pourrait rajouter MeleeB etc.
}

pub fn update_production_menu_visibility(
    q_selected_buildings: Query<&BuildingType, With<Selected>>,
    mut q_menu: Query<&mut Node, With<ProductionMenuUI>>,
) {
    let mut is_barracks_selected = false;
    for b_type in q_selected_buildings.iter() {
        if *b_type == BuildingType::Barracks {
            is_barracks_selected = true;
            break;
        }
    }
    
    if let Some(mut node) = q_menu.iter_mut().next() {
        if is_barracks_selected {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }
}

pub fn handle_production_button_clicks(
    mut interaction_query: Query<
        (&Interaction, &ProductionButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut q_selected_barracks: Query<&mut ProductionQueue, (With<Selected>, With<BuildingType>)>,
    mut economy: ResMut<PlayerEconomy>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                color.0 = Color::srgb(0.0, 0.0, 0.5); // Bleu de validation

                let unit = button.0;
                if economy.crystals >= unit.cost() {
                    // On attribue l'ordre à la première caserne sélectionnée
                    for mut queue in q_selected_barracks.iter_mut() {
                        economy.crystals -= unit.cost();
                        queue.queue.push(unit);
                        if queue.queue.len() == 1 {
                            // On démarre le timer pour la toute première unité
                            queue.timer = Timer::from_seconds(unit.build_time(), TimerMode::Once);
                        }
                        break;
                    }
                }
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.4, 0.4, 0.4);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.2, 0.2, 0.2);
            }
        }
    }
}
