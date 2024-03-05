use bevy::{math::vec3, prelude::*};

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App)  {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


// understand more query
fn camera_follow_player(
    mut cam_query: Query<(&Camera, &mut Transform), Without<Player>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (_, mut cam_transform) = cam_query.get_single_mut().unwrap();
    let player_transform = player_query.get_single_mut().unwrap();


    // delayed
    cam_transform.translation = cam_transform.translation.lerp(
        vec3(
            player_transform.translation.x,
            player_transform.translation.y,
            0.0,
        ),
        0.05,
    );
    // instant
    // cam_transform.translation = player_transform.translation;
}
