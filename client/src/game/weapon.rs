
use std::f32::consts::PI;

use macroquad::prelude::*;

use super::{Map, PlayerPosition, Geometry, Collidable, map::TILE_SIZE};
use crate::{GameObject, Resources};

pub struct Weapon {
    player_position: PlayerPosition,
    width: f32,
    height: f32,
    rotation_speed: f32,
    angle: f32,
    is_colliding: bool
}

impl Weapon {
    pub fn new(player_position: PlayerPosition, width: f32, height: f32, rotation_speed: f32) -> Self {
        Self {
            player_position,
            width,
            height,
            rotation_speed,
            angle: 0.0,
            is_colliding: false
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

        if let Some(Geometry::Circle(x, y, r)) = self.hitbox().first() {
            draw_circle_lines(*x, *y, *r, 1., if self.is_colliding { BLUE } else { RED })
        }

        draw_texture_ex(
            sword_texture,
            pos.x - sword_size.x / 2., 
            pos.y,
            WHITE, 
            DrawTextureParams { 
                dest_size: Some(sword_size),
                rotation: self.angle - PI / 2., 
                flip_y: true,
                pivot: Some(*pos),
                ..Default::default() 
            }
        );
    }
}

impl Collidable for Weapon {
    fn hitbox(&self) -> Vec<Geometry> {
        let pos = self.player_position.borrow();
        let offset = TILE_SIZE / 2.;
        let (sin, cos) = self.angle.sin_cos();

        let rotated_offset = Vec2::new(offset * cos, offset * sin);

        vec![Geometry::Circle(pos.x + rotated_offset.x, pos.y + rotated_offset.y, offset)]
    }

    fn is_colliding(&self) -> bool {
        self.is_colliding
    }

    fn set_is_colliding(&mut self, is_colliding: bool) {
        if self.is_colliding == false && is_colliding == true {
            self.change_weapon_rotation()
            
        }
       self.is_colliding = is_colliding; 
    }
}
