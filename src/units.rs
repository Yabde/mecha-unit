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

#[derive(Component)]
pub struct SelectionCollider(pub f32); // Rayon ou demi-taille pour la sélection au clic

fn setup_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Type A : Carré (Rouge)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        UnitType::TypeA,
        Speed(150.0),
        SelectionCollider(20.0), // Demi-taille du carré
    ));

    // Type B : Cercle (Vert)
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        UnitType::TypeB,
        Speed(150.0),
        SelectionCollider(20.0), // Rayon du cercle
    ));

    // Type C : Triangle (Bleu)
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 20.0),   // Haut
            Vec2::new(-20.0, -20.0), // Bas gauche
            Vec2::new(20.0, -20.0),  // Bas droit
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(100.0, 0.0, 0.0),
        UnitType::TypeC,
        Speed(150.0),
        SelectionCollider(20.0), // Distance approximative depuis le centre
    ));
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
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &mut Transform, &TargetPosition, &Speed, Option<&Selected>)>,
) {
    for (entity, mut transform, target, speed, selected) in query.iter_mut() {
        let current_pos = transform.translation.truncate(); // Récupère x et y
        let direction = target.0 - current_pos;
        let distance = direction.length();

        // ------------------
        // VISUAL DEBUG (GIZMOS)
        // ------------------
        // Si l'unité est sélectionnée, on dessine sa ligne de mouvement avec une petite croix à l'arrivée
        if selected.is_some() {
            gizmos.line_2d(current_pos, target.0, Color::WHITE);
            gizmos.circle_2d(target.0, 5.0, Color::srgb(1.0, 1.0, 0.0));
        }

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