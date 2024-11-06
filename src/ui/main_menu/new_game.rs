use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::{
    default, in_state, AlignItems, BuildChildren, ButtonBundle, Commands, Component, Entity,
    EventReader, FlexDirection, IntoSystemConfigs, JustifyContent, KeyCode, NodeBundle, OnEnter, Query, Res, ResMut, Resource, States, Style, TextBundle, TextStyle, UiRect, Val,
};
use bevy::text::Text;
#[derive(Component)]
struct ClickableArea;

pub struct NewGamePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for NewGamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            keyboard_input_new_game.run_if(in_state(self.state.clone())),
        )
        .add_systems(OnEnter(self.state.clone()), setup_new_game_ui)
        .init_resource::<ActiveInputField>();
    }
}

#[derive(Component)]
struct TextInput {
    content: String,
    active: bool, // Указывает, активно ли поле для ввода
}

#[derive(Resource, Default)]
struct ActiveInputField {
    current: Option<Entity>,
}
//TODO добавить отдельное создание персонажа и мира
pub fn setup_new_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ent: ResMut<ActiveInputField>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .with_children(|parent| {
            // Контейнер для всех элементов в виде столбца
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        height: Val::Percent(80.0),
                        width: Val::Percent(30.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Поле для ввода имени игрока
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Player Name:",
                                    TextStyle {
                                        font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                // TextInput {
                                //     content: String::new(),
                                //     active: true,
                                // },
                                // ClickableArea,
                            ));
                            let player_name_entity = parent
                                .spawn((
                                    TextBundle::from_section(
                                        "temp1", // Здесь будет отображаться имя игрока
                                        TextStyle {
                                            font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                            font_size: 20.0,
                                            color: Color::srgb(0.2, 0.6, 0.8),
                                        },
                                    ),
                                    TextInput {
                                        content: String::new(),
                                        active: false,
                                    },
                                    ClickableArea,
                                ))
                                .id();

                            ent.current = Some(player_name_entity);
                        });

                    // Поле для ввода сида мира
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "World Seed:",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn((
                                TextBundle::from_section(
                                    "temp2", // Здесь будет отображаться имя игрока
                                    TextStyle {
                                        font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                        font_size: 20.0,
                                        color: Color::srgb(0.2, 0.6, 0.8),
                                    },
                                ),
                                TextInput {
                                    content: String::new(),
                                    active: false,
                                },
                                ClickableArea,
                            ));
                        });

                    // Radiobuttons для выбора сложности
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Select Difficulty:",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));

                            let difficulties = vec!["Easy", "Medium", "Hard"];
                            for difficulty in difficulties {
                                parent
                                    .spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(80.0),
                                                height: Val::Px(30.0),
                                                margin: UiRect::top(Val::Px(5.0)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::srgb(0.3, 0.3, 0.7).into(),
                                            ..default()
                                        },
                                        DifficultyButton {
                                            level: difficulty.to_string(),
                                        },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            difficulty,
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/PIxelpointRegular.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ));
                                    });
                            }
                        });

                    // Кнопка для начала новой игры
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Px(50.0),
                                margin: UiRect::top(Val::Px(20.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.3, 0.3, 0.7).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start Game",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}

pub fn keyboard_input_new_game(
    mut active_input: ResMut<ActiveInputField>,
    mut query: Query<(Entity, &mut Text, &mut TextInput)>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    if let Some(current_entity) = active_input.current {
        if let Ok((_, mut text, mut text_input)) = query.get_mut(current_entity) {
            for event in keyboard_input_events.read() {
                if event.state == ButtonState::Released {
                    match event.key_code {
                        KeyCode::Backspace => {
                            text_input.content.pop();
                        }
                        KeyCode::Tab => {
                            text_input.active = false;
                            active_input.current = None;
                            break;
                        }
                        _ => {
                            if let Key::Character(c) = &event.logical_key {
                                text_input
                                    .content
                                    .push(c.to_string().chars().next().unwrap());
                            }
                        }
                    }
                    text.sections[0].value = text_input.content.clone();
                }
            }
        }
        if active_input.current.is_none() {
            for (entity, _, mut other_text_input) in &mut query {
                if entity == current_entity {
                    other_text_input.active = false;
                } else if !other_text_input.active {
                    other_text_input.active = true;
                    active_input.current = Some(entity);
                    break;
                }
            }
        }
    }
}

// Компонент для выбора сложности
#[derive(Component)]
struct DifficultyButton {
    level: String,
}
