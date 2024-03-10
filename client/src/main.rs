mod menu;
mod game;
mod camera;

use bevy::prelude::*;
use {menu::MenuPlugin, game::GamePlugin, camera::CameraPlugin};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    InGame
}

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1,0.0,0.15)))
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        // User configured plugins.
        .add_plugins((CameraPlugin, MenuPlugin, GamePlugin))
        .run();
}
