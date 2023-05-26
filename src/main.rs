use macroquad::prelude::*;

#[macroquad::main("")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Create game", 20. , 20., 30., GRAY);
        draw_text("Join game", 20., 40., 30., GRAY);

        next_frame().await;
    } 
}
