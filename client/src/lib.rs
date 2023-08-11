pub mod game_manager;
pub mod game;
pub mod menu;

use std::collections::HashMap;

pub use game_manager::GameManager;
pub use game::map::Map;
use macroquad::{texture::{Texture2D, load_texture}, Error};

pub type Resources = HashMap<&'static str, Texture2D>;

pub trait GameObject {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>>;
    fn draw(&self, resources: &Resources, map: &Map);
}

impl dyn GameObject {
    unsafe fn downcast<T>(&self) -> &T { 
        &*(self as *const dyn GameObject as *const T)
    }
}

pub async fn load_resources() -> Result<Resources, Error> {
   let resources = HashMap::from([
        ("weapon_sword", load_texture("./client/resources/weapon_sword_wooden.png").await?),
        ("player_anim", load_texture("./client/resources/knight_m_anim.png").await?),
    ]); 

    for resource in resources.values() {
        resource.set_filter(macroquad::texture::FilterMode::Nearest);
    }

    Ok(resources)
}

