use macroquad::prelude::*;
use macroquad_tiled::Map;

use super::{GameObject, game_entity::Position};

const PLAYER_SPRITE: u32 = 459;

pub struct Player {
    pub position: Position
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            position
        }
    }
}

impl GameObject for Player {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>> {
        let (mut x, mut y, z) = (self.position.0, self.position.1, self.position.2);

        if is_key_down(KeyCode::Up) { y -= 1 }
        if is_key_down(KeyCode::Down) { y += 1 }
        if is_key_down(KeyCode::Right) { x += 1 }
        if is_key_down(KeyCode::Left) { x -= 1 }

        if x > 0 && y > 0 && map.get_tile("collision", (x / 32) as u32, (y / 32) as u32).is_none() {
            self.position = Position(x, y, z);
        }

        None
    }

    fn draw(&self, map: &Map) {
        map.spr("tileset", PLAYER_SPRITE, Rect::new(self.position.0 as f32, self.position.1 as f32, 32., 32.));
    }
}
