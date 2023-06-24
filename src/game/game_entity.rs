use macroquad::prelude::*;

use super::GameObject;

pub struct Position (pub usize, pub usize, pub usize);

pub struct Entity {
    pub position: Position,
    pub kind: &'static str
}

impl GameObject for Entity {
    fn input(&mut self) -> Option<Box<dyn GameObject>> {
        None
    }

    fn update(&self) {
        
    }

    fn draw(&self, resources: &crate::Resources) {
        let texture = resources.get(self.kind).unwrap();
        draw_texture(*texture, self.position.0 as f32 * 32., self.position.1 as f32 * 32., WHITE);
    }
}
