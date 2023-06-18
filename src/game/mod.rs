pub mod game_entity;

use super::{GameObject, Resources};
use game_entity::{Position, Entity};

pub struct Game {
    world_entities: Vec<Entity>,
}

impl Game  {
    pub fn new() -> Self {
        let world_entities = create_entities("
            W W W W W W W W
            W . . . . . . W
            W . . . . . . W
            W . . . . . . W 
            W . . P . . . W
            W . . . . . . W
            W . . . . . . W
            W . . . . . . W
            W W W W W W W W
            ");

        Self {
            world_entities,
        }
    }
}

impl GameObject for Game {
    fn input(&mut self) -> Option<Box<dyn GameObject>> {
        None
    }

    fn update(&self) {

    }

    fn draw(&self, resources: &Resources) {
        self.world_entities.iter().for_each(|e| e.draw(&resources));
    }
}

fn create_entities(map_string: &str) -> Vec<Entity> {
    let mut entities = Vec::new();

    for (y, row) in map_string.trim().split('\n').enumerate() {
        for (x, column) in row.trim().split(' ').enumerate() {
            let position = Position(x, y, 0);
            let kind = match column {
                "W" => "wall",
                "." => "floor",
                "P" => "player",
                _ => panic!("Unrecognized map item"),
            };
            entities.push(Entity { position, kind })
        }
    }
    entities
}
