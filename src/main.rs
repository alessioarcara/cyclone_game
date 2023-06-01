use macroquad::prelude::*;

enum MenuOption {
    CreateGame,
    JoinGame,
}

struct State {
    current_option: MenuOption
}

impl State {
    fn new() -> Self {
        Self {
            current_option: MenuOption::CreateGame,
        }
    }

    fn draw(&self) {
            match self.current_option {
            MenuOption::CreateGame => {
                draw_text("Create Game", 50.0, 50.0, 30.0, WHITE);
                draw_text("Join Game", 50.0, 100.0, 30.0, GRAY);
            }
            MenuOption::JoinGame => {
                draw_text("Create Game", 50.0, 50.0, 30.0, GRAY);
                draw_text("Join Game", 50.0, 100.0, 30.0, WHITE);
            }
        }
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
            self.current_option = match self.current_option {
                MenuOption::CreateGame => MenuOption::JoinGame,
                MenuOption::JoinGame => MenuOption::CreateGame,
            };
        }
    }
}

#[macroquad::main("Arrow Game")]
async fn main() {
    let mut state = State::new();

    loop {
        clear_background(BLACK);

        state.update();
        state.draw();

        next_frame().await;
    } 
}
