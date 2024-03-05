use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Menu,
    Game
}

pub struct StatePlugin;
