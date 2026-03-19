use bevy::prelude::*;

#[derive(Component)]
pub struct ResourceNode {
    pub amount: f32, // Quantité de minerai restante
}

#[derive(Component)]
pub struct Base {
    pub team: u8,
}

#[derive(Component)]
pub struct Worker {
    pub capacity: f32,
    pub current_load: f32,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum WorkerState {
    Idle,
    MovingToResource(Entity),
    Mining(Entity),
    ReturningToBase(Entity),
}

#[derive(Component)]
pub struct MineTimer(pub Timer);
