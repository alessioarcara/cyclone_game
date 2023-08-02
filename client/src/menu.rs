use super::{GameObject, Resources, Map, game::Game};
use macroquad::prelude::*;

enum MenuOption {
    HostGame,
    JoinGame
}

impl MenuOption {
    fn to_string(&self) -> &str {
        match self {
            MenuOption::HostGame => "Host Game",
            MenuOption::JoinGame => "Join Game",
        }
    }
}

pub struct Menu {
    options: Vec<MenuOption>,
    selected_index: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: vec![MenuOption::HostGame, MenuOption::JoinGame],
            selected_index: 0,
        }
    }
}

impl GameObject for Menu {
    fn update(&mut self, _map: &Map) -> Option<Box<dyn GameObject>> {
        if is_key_pressed(KeyCode::Up) {
            self.selected_index = (self.selected_index + self.options.len() - 1) % self.options.len();
        } else if is_key_pressed(KeyCode::Down) {
            self.selected_index = (self.selected_index + 1) % self.options.len();
        } else if is_key_pressed(KeyCode::Enter) {
            match self.options[self.selected_index] {
                _ => return Some(Box::new(Game::new())),
            }
        }
        None
    }

    fn draw(&self, _resources: &Resources ,_map: &Map) {
        clear_background(BLACK);

        let (screen_width, screen_height) = (screen_width(), screen_height());
        let (text_width, text_height) = (measure_text("Host Game", None, 30, 1.).width, measure_text("Host Game", None, 30, 1.).height);
        let (center_x, center_y) = ((screen_width - text_width) / 2., (screen_height - text_height) / 2.);
        
        let option_height = 50.;
        let total_height = self.options.len() as f32 * option_height;
        let start_y = center_y - total_height / 2.0;

        for (i, option) in self.options.iter().enumerate()  {
            let y = start_y + i as f32 * option_height;

            let color = if i == self.selected_index { YELLOW } else { WHITE };
            draw_text(option.to_string(), center_x, y, 30., color)
        }
    }
}
