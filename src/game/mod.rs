pub mod game_entity;
pub mod player;

use self::player::Player;

use super::GameObject;
use macroquad::prelude::*;
use macroquad_tiled::Map;

pub struct Game {
    // world_entities: Vec<Entity>,
    player: Player
}

impl Game  {
    pub fn new() -> Self {
        //let world_entities = create_entities("
        //    W W W W W W W W
        //    W . . . . . . W
        //    W . . . . . . W
        //    W . . . . . . W 
        //    W . . P . . . W
        //    W . . . . . . W
        //    W . . . . . . W
        //    W . . . . . . W
        //    W W W W W W W W
        //    ");

        Self {
            // world_entities,
            player: Player::new(160., 160.)
        }
    }
}

impl GameObject for Game {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>> {
        self.player.update(map)
    }

    fn draw(&self, map: &Map) {
        map.draw_tiles("Tile Layer 1", Rect::new(0.0, 0.0, 320.0, 320.0), None);
        self.player.draw(map);
        map.draw_tiles("Tile Layer 2", Rect::new(0.0, 0.0, 320.0, 320.0), None);
        // self.world_entities.iter().for_each(|e| e.draw(&map));
    }
}

//fn create_entities(map_string: &str) -> Vec<Entity> {
//    let mut entities = Vec::new();
//
//    for (y, row) in map_string.trim().split('\n').enumerate() {
//        for (x, column) in row.trim().split(' ').enumerate() {
//            let position = Position(x, y, 0);
//            let kind = match column {
//                "W" => "wall",
//                "." => "floor",
//                "P" => "player",
//                _ => panic!("Unrecognized map item"),
//            };
//            entities.push(Entity { position, kind })
//        }
//    }
//    entities
//}
