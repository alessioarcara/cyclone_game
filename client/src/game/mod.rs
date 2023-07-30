pub mod map;
pub mod player;
pub mod weapon;


use self::{player::Player, weapon::Weapon};
use super::{GameObject, Resources, Map};

pub struct Game {
    player: Player
}

impl Game  {
    pub fn new() -> Self {
        Self {
            player: Player::new(160., 160.)
        }
    }
}

impl GameObject for Game {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>> {
        self.player.update(map)
    }

    fn draw(&self, resources: &Resources, map: &Map) {
        map.draw_tiles("Tile Layer 1");
        self.player.draw(resources, map);
        map.draw_tiles("Tile Layer 2");
    }
}
