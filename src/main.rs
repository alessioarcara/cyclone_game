use std::error::Error;
use arrow_game::{GameObject, GameManager, load_tilemap};
use macroquad::prelude::*;

#[macroquad::main("Cyclone Combat")]
async fn main() -> Result<(), Box<dyn Error>> {
    let map = load_tilemap().await?;
    let mut manager = GameManager::new();

    loop {
        manager.update(&map);
        manager.draw(&map);
        next_frame().await;
    } 
}

