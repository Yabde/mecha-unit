use bevy::prelude::*;

// === Couleurs des Panneaux ===
pub const PANEL_BG: Color = Color::srgba(0.1, 0.1, 0.1, 0.8);

// === Couleurs des Boutons ===
pub const BUTTON_BG: Color = Color::srgb(0.2, 0.2, 0.2);
pub const BUTTON_HOVER: Color = Color::srgb(0.4, 0.4, 0.4);
pub const BUTTON_PRESSED_BUILD: Color = Color::srgb(0.0, 0.5, 0.0);
pub const BUTTON_PRESSED_TRAIN: Color = Color::srgb(0.0, 0.0, 0.5);

// === Texte ===
pub const TEXT_SIZE_HUD: f32 = 24.0;
pub const TEXT_SIZE_BUTTON: f32 = 16.0;

// === Espacement ===
pub const PANEL_PADDING: Val = Val::Px(10.0);
pub const PANEL_GAP: Val = Val::Px(10.0);
pub const BUTTON_PADDING: Val = Val::Px(10.0);
pub const BUTTON_BORDER: Val = Val::Px(2.0);
