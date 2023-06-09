use arrow_game::{register_components, Game, initialize_level, register_resources};
use macroquad::prelude::*;
use specs::{World, WorldExt};


#[macroquad::main("Arrow Game")]
async fn main() {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world).await;

    let game = Game::new(world);

    loop {
        game.update();
        game.draw();

        next_frame().await;
    } 
}
