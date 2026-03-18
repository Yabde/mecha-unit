use bevy::prelude::*;
use bevy::camera::Viewport;
use crate::minimap::components::MinimapCamera;

pub fn setup_minimap(mut commands: Commands) {
    // Spawne simplement la caméra 2D et le marqueur. 
    // Bevy 0.15+ insère automatiquement Camera et OrthographicProjection grâce aux Required Components.
    commands.spawn((
        Camera2d,
        MinimapCamera,
    ));
}

pub fn update_minimap_viewport(
    window_query: Single<&Window>,
    mut minimap_camera: Query<(&mut Camera, &mut Projection), With<MinimapCamera>>,
) {
    let window = *window_query;
    // Dans Bevy 0.18, get_single_mut est remplacé par single_mut qui retourne un Result
    let Ok((mut camera, mut proj)) = minimap_camera.single_mut() else { return };

    // On configure la caméra ici dynamiquement
    camera.order = 1;
    camera.clear_color = ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.15));
    
    // Dans Bevy 0.18+ Projection est une enum englobant OrthographicProjection
    if let Projection::Orthographic(ref mut ortho) = *proj {
        ortho.scale = 4.0; // Zoom arrière x4 pour la minimap
    }

    let size = UVec2::new(300, 300); // Résolution de la minimap
    
    let pos_x = window.resolution.physical_width().saturating_sub(size.x);
    let pos_y = window.resolution.physical_height().saturating_sub(size.y);

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(pos_x, pos_y),
        physical_size: size,
        ..default()
    });
}
