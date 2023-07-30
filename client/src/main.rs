use std::error::Error;
use client::{GameObject, GameManager, Map, load_resources};
use macroquad::prelude::*;

#[macroquad::main("Cyclone Combat")]
async fn main() -> Result<(), Box<dyn Error>> {
    let resources = load_resources().await?;
    let map = Map::new().await?;
    let mut manager = GameManager::new();

    loop {
        manager.update(&map);
        manager.draw(&resources, &map);
        next_frame().await;
    } 
}

