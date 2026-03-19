use bevy::prelude::*;
use crate::building::components::BuildingType;
use crate::building::resources::PlacementState;
use crate::economy::components::Worker;
use crate::units::components::Selected;

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
            display: Display::None, // Caché par défaut
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        BuildMenuUI,
    )).id();

    let mut spawn_btn = |text: &str, b_type: BuildingType| {
        let btn = commands.spawn((
            Button, // Rend le composant interactif
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BuildButton(b_type),
        )).with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
        }).id();
        commands.entity(parent).add_child(btn);
    };

    spawn_btn("Caserne (100)", BuildingType::Barracks);
    spawn_btn("Tourelle (50)", BuildingType::Turret);
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
                color.0 = Color::srgb(0.0, 0.5, 0.0); // Vert de validation
                placement_state.active_building = Some(button.0);
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.4, 0.4, 0.4); // Eclairci
            }
            Interaction::None => {
                color.0 = Color::srgb(0.2, 0.2, 0.2); // Sombre base
            }
        }
    }
}
