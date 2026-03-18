use bevy::prelude::*;

// On crée un Plugin pour regrouper tout ce qui touche aux unités
pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        // On ajoute nos systèmes ici
        app.add_systems(Startup, setup_units)
           .add_systems(Update, animate_selection);
    }
}

// Nos composants (le "pub" permet de les utiliser dans d'autres fichiers)
#[derive(Component)]
pub enum UnitType { TypeA, TypeB, TypeC }

#[derive(Component)]
pub struct Selected; // Le fameux composant marqueur

fn setup_units(mut commands: Commands) {
    // Les unités (j'ai repris ton code précédent)
    commands.spawn((Sprite { color: Color::srgb(1.0, 0.0, 0.0), custom_size: Some(Vec2::new(40.0, 40.0)), ..default() }, Transform::from_xyz(-100.0, 0.0, 0.0), UnitType::TypeA));
    commands.spawn((Sprite { color: Color::srgb(0.0, 1.0, 0.0), custom_size: Some(Vec2::new(40.0, 40.0)), ..default() }, Transform::from_xyz(0.0, 0.0, 0.0), UnitType::TypeB));
    commands.spawn((Sprite { color: Color::srgb(0.0, 0.0, 1.0), custom_size: Some(Vec2::new(40.0, 40.0)), ..default() }, Transform::from_xyz(100.0, 0.0, 0.0), UnitType::TypeC));
}

// Un petit système visuel : si l'unité a le composant "Selected", on la grossit un peu !
fn animate_selection(mut query: Query<(&mut Transform, Option<&Selected>)>) {
    for (mut transform, selected) in query.iter_mut() {
        if selected.is_some() {
            transform.scale = Vec3::splat(1.3); // Grossit de 30%
        } else {
            transform.scale = Vec3::splat(1.0); // Taille normale
        }
    }
}