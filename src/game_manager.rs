use super::{GameObject, Resources, menu::Menu};

pub struct GameManager {
    state: Box<dyn GameObject>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            state: Box::new(Menu::new()),
        }
    }
}

impl GameObject for GameManager {
    fn input(&mut self) -> Option<Box<dyn GameObject>> {
        if let Some(new_state) = self.state.input() {
            self.state = new_state;
        }
        None
    }

    fn update(&self) {
        self.state.update(); 
    }
    
    fn draw(&self, resources: &Resources) {
        self.state.draw(resources);
    }
}
