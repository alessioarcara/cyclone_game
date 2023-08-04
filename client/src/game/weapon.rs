
use macroquad::prelude::*;

use super::{Map, PlayerPosition};
use crate::{GameObject, Resources};

pub struct Weapon {
    player_position: PlayerPosition,
    size: Vec2,
    rotation_speed: f32,
    angle: f32
}

impl Weapon {
    pub fn new(player_position: PlayerPosition, size: Vec2, rotation_speed: f32) -> Self {
        Self {
            player_position,
            size,
            rotation_speed,
            angle: 0.0
        }
    }

    pub fn change_weapon_rotation(&mut self) {
        self.rotation_speed = -self.rotation_speed;
    }
}

impl GameObject for Weapon {
    fn update(&mut self, _map: &Map) -> Option<Box<dyn GameObject>> {
        self.angle += self.rotation_speed;
        None 
    }

    fn draw(&self, resources: &Resources, _map: &Map) {
        let pos = self.player_position.borrow();
        let sword_texture = resources.get("weapon_sword").unwrap();
        let sword_size = vec2(sword_texture.width() * 1.5, sword_texture.height() * 1.5);

        draw_texture_ex(sword_texture,
            pos.x - sword_size.x / 2., 
            pos.y ,
            WHITE, 
            DrawTextureParams { 
                dest_size: Some(sword_size),
                rotation: self.angle, 
                flip_y: true,
                pivot: Some(*pos),
                ..Default::default() 
            }
        );
    }
}
