use std::{rc::Rc, cell::RefCell};

use macroquad::prelude::{*, animation::AnimatedSprite, animation::Animation};

use super::{Weapon, map::TILE_SIZE, Collidable, Geometry};
use crate::{GameObject, Map, Resources};

const RADIUS: f32 = TILE_SIZE / 2.;

pub type PlayerPosition = Rc<RefCell<Vec2>>;

pub struct Player {
    pub position: PlayerPosition,
    is_colliding: bool,
    weapon: Weapon,
    sprite: AnimatedSprite,
    animation_state: AnimationState
}

#[derive(Default)]
enum AnimationState {
    #[default]
    Idle,
    Moving,
}

impl Player {
    pub fn new(position: PlayerPosition, weapon: Weapon) -> Self {
        Self {
            position,
            is_colliding: false,
            weapon,
            sprite: AnimatedSprite::new(
                16, 
                28, 
                &[ 
                    Animation {
                        name: "idle".to_string(),
                        row:  0,
                        frames: 4,
                        fps: 4
                    },
                    Animation {
                        name: "run".to_string(),
                        row:  1,
                        frames: 4,
                        fps: 15
                    }
                ], 
                true
            ),
            animation_state: AnimationState::default()
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

        if x != pos.x || y != pos.y {
            self.animation_state = AnimationState::Moving;
        } else {
            self.animation_state = AnimationState::Idle;
        }

        match self.animation_state {
            AnimationState::Idle => self.sprite.set_animation(0),
            AnimationState::Moving => self.sprite.set_animation(1),
        }

        if map.in_bounds(x, y) && map.can_enter_tile(x, y) {
            pos.x = x;
            pos.y = y;
        }

        if is_key_pressed(KeyCode::Space) { self.weapon.change_weapon_rotation() }

        self.weapon.update(map);
        self.sprite.update();
        None
    }

    fn draw(&self, resources: &Resources, map: &Map) {
        let pos = self.position.borrow();
        draw_circle_lines(pos.x, pos.y, RADIUS, 1., if self.is_colliding { BLUE } else { RED });
        draw_texture_ex(
            &resources["player_anim"], 
            pos.x - RADIUS,
            pos.y - (56. / 4. * 3.) , 
            WHITE, 
            DrawTextureParams { 
                dest_size: Some(2. * self.sprite.frame().dest_size), 
                source: Some(self.sprite.frame().source_rect), 
                ..Default::default()
            });
        self.weapon.draw(resources, map);
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Vec<Geometry> {
        let pos = self.position.borrow();
        let mut a = vec![Geometry::Circle(pos.x, pos.y, RADIUS)];
        a.extend(self.weapon.hitbox());
        a
    }

    fn is_colliding(&self) -> bool {
        self.is_colliding
    }

    fn set_is_colliding(&mut self, is_colliding: bool) {
        self.is_colliding = is_colliding;
        self.weapon.set_is_colliding(is_colliding);
    }
}
