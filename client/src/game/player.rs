use bevy::{prelude::*, utils::warn};

use crate::GameState;

// leverage predefined bevy built-in
#[derive(Component, Debug)]
pub struct Position {
    pub value: Vec2
}

#[derive(Component)]
struct AnimationIndices {
    first: usize, 
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player);
        app.add_systems(Update, (animate_sprite, move_player).run_if(in_state(GameState::InGame)));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = asset_server.load("knight_m_idle_anim.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 32.), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 3 };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        }, 
        Player {}
    ));
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += 1.;
        } else if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= 1.;
        } else if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += 1.;
        } else if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= 1.;
        }
    } 
}

