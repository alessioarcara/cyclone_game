pub mod game;
pub mod menu;

use menu::Menu;

pub trait GameObject {
    fn input(&mut self);
    fn update(&self);
    fn draw(&self);
}

pub struct GameManager {
    state: Box<dyn GameObject> 
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            state: Box::new(Menu::new()),
        }
    }
}

impl GameObject for GameManager {
    fn input(&mut self) {
        self.state.input();
    }

    fn update(&self) {
        self.state.update(); 
    }

    fn draw(&self) {
        self.state.draw();   
    }
}

//pub fn initialize_level() {
//    const MAP: &str = "
//    W W W W W W W W
//    W . . . . . . W
//    W . . . . . . W
//    W . . . . . . W 
//    W . . P . . . W
//    W . . . . . . W
//    W . . . . . . W
//    W . . . . . . W
//    W W W W W W W W
//    ";
//    load_map(MAP.to_string());
//}

//pub fn load_map(map_string: String) {
   // let wall_texture = load_texture("resources/wall.png").await.expect("Failed to load resource");
   // let floor_texture = load_texture("resources/floor.png").await.expect("Failed to load resource");
   // let player_texture = load_texture("resources/player.png").await.expect("Failed to load resource");
   // let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

   // for (y, &row) in rows.iter().enumerate() {
   //     let columns: Vec<&str> = row.split(' ').collect();

   //     for (x, &column) in columns.iter().enumerate() {
   //         let x = x as f32;
   //         let y = y as f32;
   //         let position = Position { x, y, z: 0 };

   //        match column {
   //             "." => create_floor(world, position, floor_texture),
   //             "W" => create_wall(world, position, wall_texture),
   //             "P" => {
   //                 create_floor(world, position, floor_texture);
   //                 create_player(world, Position { x, y, z: 1 }, player_texture);
   //             },
   //             _ => panic!("unrecognized map item")
   //         } 
   //     }
   // }
//}
