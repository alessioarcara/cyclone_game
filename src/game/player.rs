use std::{f32::consts::PI, rc::Rc, cell::RefCell};

use macroquad::prelude::*;
use macroquad_tiled::Map;

use super::GameObject;

const PLAYER_SPRITE: u32 = 459;

type PlayerPosition = Rc<RefCell<Vec2>>;

pub struct Player {
    position: PlayerPosition, 
    weapon: Weapon
}

pub struct Weapon {
    player_position: PlayerPosition,
    size: Vec2,
    rotation_speed: f32,
    angle: f32
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Rc::new(RefCell::new(Vec2::new(x, y)));
        Self {
            position: position.clone(),
            weapon: Weapon::new(
                position.clone(),
                Vec2::new(30., 10.),
                2. * PI / 180.
            )
        }
    }
}

impl GameObject for Player {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>> {
        let mut pos = self.position.borrow_mut();
        let (mut x, mut y) = (pos.x, pos.y);

        if is_key_down(KeyCode::Up) { y -= 1. }
        if is_key_down(KeyCode::Down) { y += 1. }
        if is_key_down(KeyCode::Right) { x += 1. }
        if is_key_down(KeyCode::Left) { x -= 1. }

        if x > 0. && y > 0. && map.get_tile("collision", (x / 32.) as u32, (y / 32.) as u32).is_none() {
            pos.x = x;
            pos.y = y;
        }

        self.weapon.update(map);
        None
    }

    fn draw(&self, map: &Map) {
        let pos = self.position.borrow();
        map.spr("tileset", PLAYER_SPRITE, Rect::new(pos.x - 16., pos.y - 16., 32., 32.));
        self.weapon.draw(map);
    }
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
}

impl GameObject for Weapon {
    fn update(&mut self, _map: &Map) -> Option<Box<dyn GameObject>> {
        self.angle += self.rotation_speed;
        None 
    }

    fn draw(&self, _map: &Map) {
        let player_pos = self.player_position.borrow();
        let offset = 32.;
        let (hx, hy) = (self.size.x / 2., self.size.y / 2.);
        let (sin, cos) = self.angle.sin_cos();

        let rotated_offset = Vec2::new(offset * cos, offset * sin);

        let weapon_center = *player_pos + rotated_offset;

        let corners = [
            Vec2::new(-hx, -hy),  // top left
            Vec2::new(hx, -hy),   // top right
            Vec2::new(hx, hy),    // bottom right
            Vec2::new(-hx, hy),   // bottom left
        ];

        let rotated_corners: Vec<Vec2> = corners.iter().map(|&corner| {
            weapon_center + Vec2::new(
                corner.x * cos - corner.y * sin,
                corner.x * sin + corner.y * cos,
            )
        }).collect();

        for i in 0..4 {
            draw_line(rotated_corners[i].x, rotated_corners[i].y, 
                rotated_corners[(i+1)%4].x, rotated_corners[(i+1)%4].y, 
                2., RED);
        }
    }
}
