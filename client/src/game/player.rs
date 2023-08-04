use std::{rc::Rc, cell::RefCell};

use macroquad::prelude::*;

use super::{Weapon, map::TILE_SIZE, Collidable, Geometry};
use crate::{GameObject, Map, Resources};

const PLAYER_SPRITE: u32 = 459;
const RADIUS: f32 = TILE_SIZE / 2.;

pub type PlayerPosition = Rc<RefCell<Vec2>>;

pub struct Player {
    pub position: PlayerPosition,
    is_colliding: bool,
    weapon: Weapon
}

impl Player {
    pub fn new(position: PlayerPosition, weapon: Weapon) -> Self {
        Self {
            position,
            is_colliding: false,
            weapon
        }
    }

    pub fn set_position(&self, x: f32, y: f32) {
        let mut pos = self.position.borrow_mut();
        pos.x = x;
        pos.y = y;
    }
}

impl GameObject for Player  {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>> {
        let mut pos = self.position.borrow_mut();
        let (mut x, mut y) = (pos.x, pos.y);

        if is_key_down(KeyCode::Up) { y -= 1. }
        if is_key_down(KeyCode::Down) { y += 1. }
        if is_key_down(KeyCode::Right) { x += 1. }
        if is_key_down(KeyCode::Left) { x -= 1. }

        if map.in_bounds(x, y) && map.can_enter_tile(x, y) {
            pos.x = x;
            pos.y = y;
        }

        if is_key_pressed(KeyCode::Space) { self.weapon.change_weapon_rotation() }
        self.weapon.update(map);
        None
    }

    fn draw(&self, resources: &Resources, map: &Map) {
        if let Geometry::Circle(x, y, r) = self.hitbox() {
            draw_circle_lines(x, y, r, 1., if self.is_colliding { BLUE } else { RED });
            map.spr("tileset", PLAYER_SPRITE, Rect::new(x - RADIUS, y - RADIUS, TILE_SIZE, TILE_SIZE));
            self.weapon.draw(resources, map);
        }
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Geometry {
        let pos = self.position.borrow();
        Geometry::Circle(pos.x, pos.y, RADIUS)
    }

    fn is_colliding(&self) -> bool {
        self.is_colliding
    }

    fn set_is_colliding(&mut self, is_colliding: bool) {
        self.is_colliding = is_colliding;
    }
}
