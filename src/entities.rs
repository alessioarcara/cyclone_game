use macroquad::texture::Texture2D;
use specs::{World, WorldExt, Builder};

use crate::components::{Position, Wall, Renderable};

pub fn create_wall(world: &mut World, position: Position, texture: Texture2D) {
    world 
        .create_entity()
        .with(position)
        .with(Renderable { texture })
        .with(Wall{})
        .build();
}

pub fn create_floor(world: &mut World, position: Position, texture: Texture2D) {
    world 
        .create_entity()
        .with(position)
        .with(Renderable { texture })
        .build();
}
