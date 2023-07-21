pub mod game_manager;
pub mod game;
pub mod menu;

use macroquad::{prelude::*, Error};
pub use game_manager::GameManager;
use macroquad_tiled::Map;

pub trait GameObject {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>>;
    fn draw(&self, map: &Map);
}

pub async fn load_tilemap() -> Result<Map, Error> {
    let dungeon_tileset = load_texture("./resources/tileset.png").await?;
    let collision_tileset = load_texture("./resources/collision_graphic.png").await?;
    dungeon_tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("./resources/tilemap.tmj").await?;

    let map = macroquad_tiled::load_map(
        &tiled_map_json, 
        &[("tileset.png", dungeon_tileset), ("collision_graphic.png", collision_tileset)], 
        &[]
    ).expect("failed to load map");

    Ok(map)
}
