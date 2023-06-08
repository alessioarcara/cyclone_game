use macroquad::prelude::*;
use specs::{System, ReadStorage, Join};

use crate::components::{Position, Renderable};

pub struct RenderingSystem;

const TILE_SIZE: f32 = 32.0;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        clear_background(BLACK);

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (&position, renderable) in rendering_data.iter() {
            let x = position.x * TILE_SIZE;
            let y = position.y * TILE_SIZE;
            
            draw_texture(
                renderable.texture, 
                x,
                y,
                WHITE, 
            )
        }
    }
}
