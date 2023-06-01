use specs::{Component, VecStorage };

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: u8
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}
