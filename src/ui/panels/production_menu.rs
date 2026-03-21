use bevy::prelude::*;
use crate::building::components::{BuildingType, ProductionQueue};
use crate::units::components::UnitType;
use crate::core::components::Selected;
use crate::economy::resources::PlayerEconomy;
use crate::ui::theme;
use crate::ui::widgets::button::spawn_action_button;

#[derive(Component)]
pub struct ProductionMenuUI;

#[derive(Component)]
pub struct ProductionButton(pub UnitType);

pub fn setup_production_menu(mut commands: Commands) {
    let parent = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            right: Val::Px(350.0),
            display: Display::None,
            flex_direction: FlexDirection::Row,
            row_gap: theme::PANEL_GAP,
            column_gap: theme::PANEL_GAP,
            padding: UiRect::all(theme::PANEL_PADDING),
            ..default()
        },
        BackgroundColor(theme::PANEL_BG),
        Interaction::None,
        ProductionMenuUI,
    )).id();

    spawn_action_button(&mut commands, parent, "Worker (50)", ProductionButton(UnitType::Worker));
    spawn_action_button(&mut commands, parent, "Guerrier (100)", ProductionButton(UnitType::MeleeA));
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
                color.0 = theme::BUTTON_PRESSED_TRAIN;

                let unit = button.0;
                if economy.crystals >= unit.cost() {
                    for mut queue in q_selected_barracks.iter_mut() {
                        economy.crystals -= unit.cost();
                        queue.queue.push(unit);
                        if queue.queue.len() == 1 {
                            queue.timer = Timer::from_seconds(unit.build_time(), TimerMode::Once);
                        }
                        break;
                    }
                }
            }
            Interaction::Hovered => {
                color.0 = theme::BUTTON_HOVER;
            }
            Interaction::None => {
                color.0 = theme::BUTTON_BG;
            }
        }
    }
}
