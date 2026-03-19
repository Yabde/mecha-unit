use bevy::prelude::*;

mod units;
mod input;
mod combat;
mod minimap;
mod economy;

// Marqueur pour distinguer la caméra principale du jeu de celle de la minimap
#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        // Gris "pierre" clair pour le fond de la carte
        .insert_resource(ClearColor(Color::srgb(0.55, 0.55, 0.6)))
        
        .add_plugins(DefaultPlugins)

        // On ajoute nos propres plugins métier
        .add_plugins(units::UnitsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(minimap::MinimapPlugin)
        .add_plugins(economy::EconomyPlugin)

        .add_systems(Startup, setup_camera)
        
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d, 
        MainCamera,
    ));
}