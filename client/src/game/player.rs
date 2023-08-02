use std::{f32::consts::PI, rc::Rc, cell::RefCell};

use macroquad::prelude::*;

use super::{GameObject, Resources, Map, Weapon, map::TILE_SIZE, geometry::{Collidable, Geometry}};

const PLAYER_SPRITE: u32 = 459;
const RADIUS: f32 = TILE_SIZE / 2.;

pub type PlayerPosition = Rc<RefCell<Vec2>>;

pub struct Player {
    pub position: PlayerPosition,
    pub is_colliding: bool,
    weapon: Weapon,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Rc::new(RefCell::new(Vec2::new(x, y)));
        Self {
            position: position.clone(),
            is_colliding: false,
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

        }
        self.weapon.draw(resources, map);
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Geometry {
        let pos = self.position.borrow();
        Geometry::Circle(pos.x, pos.y, RADIUS)
    }
}
