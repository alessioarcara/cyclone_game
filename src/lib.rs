use core::panic;

use components::{Position, Renderable, Player, Wall};
use entities::{create_floor, create_wall};
use macroquad::texture::load_texture;
use specs::{World, WorldExt, RunNow};
use systems::RenderingSystem;

mod components;
mod entities;
mod systems;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
}

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(world: World) -> Self {
        Self {
            world,
        }
    }

    pub fn update(&self) {

    }

    pub fn draw(&self) {
        {
            let mut rs =  RenderingSystem;
            rs.run_now(&self.world)
        }
    }
}

pub async fn initialize_level(world: &mut World) {
    const MAP: &str = "
    W W W W W W W W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W 
    W . . . . . . W
    W . . . . . . W
    W . . . . . . W
    W . . . . . . W
    W W W W W W W W
    ";
    load_map(world, MAP.to_string()).await;
}

pub async fn load_map(world: &mut World, map_string: String) {
    let wall_texture = load_texture("../resources/wall.png").await.expect("Failed to load resource");
    let floor_texture = load_texture("../resources/floor.png").await.expect("Failed to load resource");
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, &row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, &column) in columns.iter().enumerate() {
            let position = Position { x: x as f32, y: y as f32, z: 0 };

           match column {
                "." => create_floor(world, position, floor_texture),
                "W" => create_wall(world, position, wall_texture),
                _ => panic!("unrecognized map item")
            } 
        }
    }
}
