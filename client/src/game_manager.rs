use super::{GameObject, Resources, Map, menu::Menu};

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
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>> {
        if let Some(new_state) = self.state.update(map) {
            self.state = new_state;
        }
        None
    }
    
    fn draw(&self, resources: &Resources, map: &Map) {
        self.state.draw(resources, map);
    }
}
