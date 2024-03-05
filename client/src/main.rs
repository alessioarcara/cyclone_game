mod camera;
mod player;

use bevy::prelude::*;

use camera::CameraPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1,0.0,0.15)))
        // User configured plugins.
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
