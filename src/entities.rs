use specs::{World, WorldExt, Builder};

use crate::components::{Position, Wall, Renderable};

pub fn create_wall(world: &mut World, position: Position) {
    world 
        .create_entity()
        .with(position)
        .with(Renderable {
            path: "wall".to_string()
        })
        .with(Wall{})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world 
        .create_entity()
        .with(position)
        .with(Renderable {
            path: "floor".to_string()
        })
        .build();
}
