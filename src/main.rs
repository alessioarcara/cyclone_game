use arrow_game::{register_components, Game, initialize_level};
use macroquad::prelude::*;
use specs::{World, WorldExt};


#[macroquad::main("Arrow Game")]
async fn main() {
    let mut world = World::new();
    register_components(&mut world);
    initialize_level(&mut world).await;

    let game = Game::new(world);
    loop {
        game.update();
        game.draw();

        next_frame().await;
    } 
}
