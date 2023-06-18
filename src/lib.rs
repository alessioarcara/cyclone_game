pub mod game_manager;
pub mod game;
pub mod menu;

use std::collections::HashMap;
use macroquad::prelude::*;
pub use game_manager::GameManager;

type Resources = HashMap<&'static str, Texture2D>;

pub trait GameObject {
    fn input(&mut self) -> Option<Box<dyn GameObject>>;
    fn update(&self);
    fn draw(&self, resources: &Resources);
}

pub async fn load_resources() -> Result<Resources, FileError> {
    let mut resources = HashMap::new();

    resources.insert("wall", load_texture("../resources/wall.png").await?);
    resources.insert("floor", load_texture("../resources/floor.png").await?);
    resources.insert("player", load_texture("../resources/player.png").await?);

    Ok(resources)
}
