use macroquad::{prelude::*, Error};

pub const TILE_SIZE: f32 = 32.;

pub struct Map {
    map: macroquad_tiled::Map 
}

impl Map {
   pub async fn new() -> Result<Map, Error> {
        let dungeon_tileset = load_texture("./client/resources/tileset.png").await?;
        let collision_tileset = load_texture("./client/resources/collision_graphic.png").await?;
        dungeon_tileset.set_filter(FilterMode::Nearest);

        let tiled_map_json = load_string("./client/resources/tilemap.tmj").await?;

        let map = macroquad_tiled::load_map(
            &tiled_map_json, 
            &[("tileset.png", dungeon_tileset), ("collision_graphic.png", collision_tileset)], 
            &[]
        ).expect("failed to load map");

        Ok(Map { map })
    } 

    pub fn in_bounds(&self, x: f32, y: f32) -> bool {
        x > 0. && y > 0.
    }

    pub fn can_enter_tile(&self, x: f32, y: f32) -> bool {
        self.map.get_tile("collision", (x / TILE_SIZE) as u32, (y / TILE_SIZE) as u32).is_none() 
    }

    pub fn draw_tiles(&self, layer: &str) {
        self.map.draw_tiles(layer, Rect::new(0.0, 0.0, 320., 320.), None);
    }

    pub fn spr(&self, tileset: &str, sprite: u32, dest: Rect) {
        self.map.spr(tileset, sprite, dest)
    }
}
