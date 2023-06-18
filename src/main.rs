use std::error::Error;
use arrow_game::{GameObject, GameManager, load_resources};
use macroquad::prelude::*;

#[macroquad::main("Arrow Game")]
async fn main() -> Result<(), Box<dyn Error>> {
    let resources = load_resources().await?;
    let mut manager = GameManager::new();

    loop {
        manager.input();
        manager.update();
        manager.draw(&resources);
        next_frame().await;
    } 
}

