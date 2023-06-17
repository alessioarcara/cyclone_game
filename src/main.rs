use arrow_game::{GameObject, GameManager};
use macroquad::prelude::*;

#[macroquad::main("Arrow Game")]
async fn main() {
    let mut manager = GameManager::new();

    loop {
        manager.input();
        manager.update();
        manager.draw();
        next_frame().await;
    } 
}
