use bevy::prelude::*;

// On crée un Plugin pour regrouper tout ce qui touche aux unités
pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        // On ajoute nos systèmes ici
        app.add_systems(Startup, setup_units)
           .add_systems(Update, (animate_selection, move_units));
    }
}

// Nos composants (le "pub" permet de les utiliser dans d'autres fichiers)
#[derive(Component)]
pub enum UnitType { TypeA, TypeB, TypeC }

#[derive(Component)]
pub struct Selected; // Le fameux composant marqueur

#[derive(Component)]
pub struct Speed(pub f32); // Vitesse de déplacement

#[derive(Component)]
pub struct TargetPosition(pub Vec2); // La destination finale

fn setup_units(mut commands: Commands) {
    // On ajoute le composant Speed à nos unités lors de leur création
    commands.spawn((Sprite { color: Color::srgb(1.0, 0.0, 0.0), custom_size: Some(Vec2::new(40.0, 40.0)), ..default() }, Transform::from_xyz(-100.0, 0.0, 0.0), UnitType::TypeA, Speed(150.0)));
    commands.spawn((Sprite { color: Color::srgb(0.0, 1.0, 0.0), custom_size: Some(Vec2::new(40.0, 40.0)), ..default() }, Transform::from_xyz(0.0, 0.0, 0.0), UnitType::TypeB, Speed(150.0)));
    commands.spawn((Sprite { color: Color::srgb(0.0, 0.0, 1.0), custom_size: Some(Vec2::new(40.0, 40.0)), ..default() }, Transform::from_xyz(100.0, 0.0, 0.0), UnitType::TypeC, Speed(150.0)));
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

// Système qui déplace toutes les unités possédant `TargetPosition`
fn move_units(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &TargetPosition, &Speed)>,
) {
    for (entity, mut transform, target, speed) in query.iter_mut() {
        let current_pos = transform.translation.truncate(); // Récupère x et y
        let direction = target.0 - current_pos;
        let distance = direction.length();

        // Si l'unité est assez proche (marge d'erreur de 2 pixels), elle a atteint sa cible
        if distance < 2.0 {
            transform.translation.x = target.0.x;
            transform.translation.y = target.0.y;
            
            // On retire le composant TargetPosition puisqu'elle est arrivée
            commands.entity(entity).remove::<TargetPosition>();
        } else {
            // Sinon, on calcule le vecteur de déplacement normalisé (direction) * vitesse * dt
            let movement = direction.normalize() * speed.0 * time.delta_secs();
            transform.translation += movement.extend(0.0);
        }
    }
}