use bevy::{camera::visibility::RenderLayers, prelude::*};

mod units;
mod input;
mod combat;
mod minimap;

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

        // Configurer les Gizmos pour ne s'afficher que sur le Layer 2
        .add_systems(Startup, (setup_camera, configure_gizmos))
        
        .run();
}

fn configure_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.render_layers = RenderLayers::layer(2);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d, 
        MainCamera,
        // La caméra principale voit le monde (Layer 0 par défaut) ET les gizmos (Layer 2)
        RenderLayers::from_layers(&[0, 2]),
    ));
}