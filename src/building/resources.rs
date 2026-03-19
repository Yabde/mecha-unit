use bevy::prelude::*;
use super::components::BuildingType;

#[derive(Resource, Default)]
pub struct PlacementState {
    pub active_building: Option<BuildingType>,
}
