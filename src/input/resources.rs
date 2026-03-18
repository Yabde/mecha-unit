use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectionState {
    pub start_pos: Option<Vec2>,
    pub end_pos: Option<Vec2>,
}
