use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::asset::RenderAssetUsages;
use bevy::camera::RenderTarget;
use crate::minimap::components::MinimapCamera;

pub fn setup_minimap(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // 1. Créer une texture 300x300 qui servira de cible de rendu (RenderTarget)
    let size = Extent3d {
        width: 300,
        height: 300,
        depth_or_array_layers: 1,
    };
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // On indique à Bevy qu'elle peut être utilisée comme cible de caméra et pour être lue dans l'UI
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);

    // 2. Créer la caméra de minimap qui dessine EXCLUSIVEMENT dans cette texture (indépendant de la fenêtre)
    commands.spawn((
        Camera2d,
        Camera {
            order: -1, // L'ordre n'a plus d'importance car elle rend dans une texture isolée
            // On utilise la même couleur "gris pierre" que le monde principal, mais avec 80% d'opacité
            clear_color: ClearColorConfig::Custom(Color::srgba(0.55, 0.55, 0.6, 0.8)),
            ..default()
        },
        RenderTarget::Image(image_handle.clone().into()),
        Projection::Orthographic(OrthographicProjection {
            scale: 14.0, // Zoom arriere pour couvrir la carte 4000x3000
            ..OrthographicProjection::default_2d()
        }),
        MinimapCamera,
    ));

    // 3. Spawne l'interface utilisateur pour afficher la texture avec une bordure.
    // L'avantage est que l'UI Engine de Bevy s'occupe du positionnement et du DPI Scaling tout seul !
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0), // On décale de 10px du bord droit pour faire plus joli
            bottom: Val::Px(10.0), // 10px du bord bas
            width: Val::Px(308.0), // Bordure (300 + 4px + 4px)
            height: Val::Px(308.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        // Bordure légèrement transparente (50% d'opacité)
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5)),
    )).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(300.0),
                ..default()
            },
            ImageNode::new(image_handle),
        ));
    });
}
