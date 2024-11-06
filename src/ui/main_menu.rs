mod main_screen;
pub mod new_game;
mod settings;
use crate::ui::main_menu::main_screen::{setup_main_screen, WhatAButton};
use crate::ui::main_menu::new_game::NewGamePlugin;
use crate::ui::setup_ui_camera;
use crate::MainMenuState;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::OnExit;
use bevy::prelude::{
    in_state, BackgroundColor, Button, Changed, Commands, Component, Entity, Interaction, Node,
    Query, States, With,
};
use bevy::prelude::{AppExtStates, NextState, OnEnter, Reflect, ResMut};

pub struct MyMainMenuPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MyMainMenuPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system.run_if(in_state(self.state.clone())))
            .add_systems(Startup, setup_ui_camera)
            .add_systems(OnEnter(MainMenuState::MainScreen), setup_main_screen)
            .add_systems(OnExit(MainMenuState::MainScreen), despawn_ui)
            .insert_state(MainMenuState::MainScreen)
            .add_plugins(NewGamePlugin {
                state: MainMenuState::NewGame,
            });
    }
}

pub enum MenuButton {
    MainScreenButton(WhatAButton),
}
#[derive(Component)]
pub struct ButtonType {
    button_type: MenuButton,
}
pub fn despawn_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for i in query.iter() {
        commands.entity(i).despawn_recursive();
    }
}

// Система для обработки взаимодействий с кнопкой
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match &button_type.button_type {
                    MenuButton::MainScreenButton(but) => {
                        match but {
                            WhatAButton::Play => {
                                // Переход к под-состоянию NewGame в MainMenuState
                                next_state.set(MainMenuState::NewGame);

                                println!("Play button clicked!");
                            }
                            WhatAButton::Settings => {
                                println!("Settings button clicked!");
                            }
                            WhatAButton::Github => {
                                println!("Github button clicked!");
                            }
                            WhatAButton::Exit => {
                                println!("Exit button clicked!");
                            }
                            WhatAButton::LoadGame => {
                                println!("LoadGame button clicked!");
                            }
                        }
                    }
                }
                // Обработка события нажатия
                *color = BackgroundColor(Color::srgb(0.7, 0.7, 0.9));
            }
            Interaction::Hovered => {
                // Обработка наведения курсора на кнопку
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.9));
            }
            Interaction::None => {
                // Возвращаем цвет к исходному, когда взаимодействие прекращается
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.7));
            }
        }
    }
}
