use bevy::prelude::*;

mod core;

mod units;
mod input;
mod combat;
mod minimap;
mod economy;
mod ui;
mod factory;
mod building;
mod map;

// Marqueur pour distinguer la camera principale du jeu de celle de la minimap
#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        // Gris "pierre" clair pour le fond de la carte
        .insert_resource(ClearColor(Color::srgb(0.55, 0.55, 0.6)))
        
        .add_plugins(DefaultPlugins)

        // On ajoute nos propres plugins metier
        .add_plugins(units::UnitsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(minimap::MinimapPlugin)
        .add_plugins(economy::EconomyPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(building::BuildingPlugin)
        .add_plugins(map::MapPlugin)

        .add_systems(Startup, setup_camera)
        
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d, 
        MainCamera,
        // Camera commence sur la base du joueur
        Transform::from_xyz(-1600.0, -1100.0, 0.0),
        Projection::Orthographic(OrthographicProjection::default_2d()),
    ));
}