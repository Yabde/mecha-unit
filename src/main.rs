use bevy::prelude::*;

mod units;
mod input;
mod combat;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // On ajoute nos propres plugins métier
        .add_plugins(units::UnitsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(combat::CombatPlugin)

        // On garde la caméra ici car elle est globale au jeu
        .add_systems(Startup, setup_camera)
        
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}