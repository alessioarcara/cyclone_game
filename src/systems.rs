use macroquad::prelude::*;
use specs::{System, ReadStorage, Join, Write, WriteStorage};

use crate::{components::{Position, Renderable, Player}, DeltaTime};

pub struct RenderingSystem;
pub struct InputSystem;

const TILE_SIZE: f32 = 32.0;
const SPEED: f64 = 0.1;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        clear_background(WHITE);

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

impl<'a> System<'a> for InputSystem {
    type SystemData = (
    Write<'a, DeltaTime>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Player>,
);

    fn run(&mut self, data: Self::SystemData) {
        let (mut delta, mut positions, players) = data;

        let current_time = get_time();
        let time_difference = current_time - delta.0;

        for (position, _player) in (&mut positions, &players).join() {
            let x = if is_key_down(KeyCode::Left) { -1.0 } else if is_key_down(KeyCode::Right) { 1.0 } else { 0.0 };
            let y = if is_key_down(KeyCode::Up) { 1.0 } else if is_key_down(KeyCode::Down) { -1.0 } else { 0.0 };

            if time_difference > SPEED {
                delta.0 = current_time;
                position.x += x;
                position.y -= y;
            }
        }
    }
}
