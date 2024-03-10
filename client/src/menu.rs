use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnExit(GameState::Menu), despawn_screen::<Menu>)
            .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)));
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
enum MenuButton {
    Play,
    Quit
}

fn menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent 
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButton::Play, "Play"), 
                        (MenuButton::Quit, "Quit")
                    ] {
                            parent 
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        ..default()
                                    },
                                    action,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        text, 
                                        button_text_style.clone()
                                    ));
                                });
                        }
                });
        });
}

fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButton),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButton::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButton::Play => {
                    game_state.set(GameState::InGame)
                }
            }
        }
    }
}

fn despawn_screen<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
