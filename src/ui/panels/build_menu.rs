use bevy::prelude::*;
use crate::building::components::BuildingType;
use crate::building::resources::PlacementState;
use crate::economy::components::Worker;
use crate::units::components::Selected;
use crate::ui::theme;
use crate::ui::widgets::button::spawn_action_button;

#[derive(Component)]
pub struct BuildMenuUI;

#[derive(Component)]
pub struct BuildButton(pub BuildingType);

pub fn setup_build_menu(mut commands: Commands) {
    let parent = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            left: Val::Px(15.0),
            display: Display::None,
            flex_direction: FlexDirection::Row,
            row_gap: theme::PANEL_GAP,
            column_gap: theme::PANEL_GAP,
            padding: UiRect::all(theme::PANEL_PADDING),
            ..default()
        },
        BackgroundColor(theme::PANEL_BG),
        Interaction::None,
        BuildMenuUI,
    )).id();

    spawn_action_button(&mut commands, parent, "Caserne (100)", BuildButton(BuildingType::Barracks));
    spawn_action_button(&mut commands, parent, "Tourelle (50)", BuildButton(BuildingType::Turret));
}

pub fn update_build_menu_visibility(
    q_selected_workers: Query<&Worker, With<Selected>>,
    mut q_menu: Query<&mut Node, With<BuildMenuUI>>,
) {
    let has_worker_selected = !q_selected_workers.is_empty();
    
    if let Some(mut node) = q_menu.iter_mut().next() {
        if has_worker_selected {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }
}

pub fn handle_build_button_clicks(
    mut interaction_query: Query<
        (&Interaction, &BuildButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut placement_state: ResMut<PlacementState>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                color.0 = theme::BUTTON_PRESSED_BUILD;
                placement_state.active_building = Some(button.0);
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
