use crate::ui::main_menu::ButtonType;
use crate::ui::main_menu::MenuButton::MainScreenButton;
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::prelude::{
    default, AlignItems, BuildChildren, ButtonBundle, Commands, FlexDirection, ImageBundle,
    JustifyContent, NodeBundle, PositionType, Res, Style, TextBundle, TextStyle, UiImage, Val,
};

pub enum WhatAButton {
    Play,
    Settings,
    Github,
    Exit,
    LoadGame,
}
pub fn setup_main_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            // Center buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        height: Val::Percent(50.0),
                        width: Val::Percent(30.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // "Play" button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(80.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center, // Центрирует по горизонтали
                                    align_items: AlignItems::Center, // Центрирует по вертикали
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.3, 0.7).into(),

                                ..default()
                            },
                            ButtonType {
                                button_type: MainScreenButton(WhatAButton::Play),
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // "Load Game" button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(80.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center, // Центрирует по горизонтали
                                    align_items: AlignItems::Center, // Центрирует по вертикали
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.3, 0.7).into(),
                                ..default()
                            },
                            ButtonType {
                                button_type: MainScreenButton(WhatAButton::LoadGame),
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Load Game",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // "Settings" button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(80.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center, // Центрирует по горизонтали
                                    align_items: AlignItems::Center, // Центрирует по вертикали
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.3, 0.7).into(),
                                ..default()
                            },
                            ButtonType {
                                button_type: MainScreenButton(WhatAButton::Settings),
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    // "Exit" button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(80.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center, // Центрирует по горизонтали
                                    align_items: AlignItems::Center, // Центрирует по вертикали
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.3, 0.7).into(),
                                ..default()
                            },
                            ButtonType {
                                button_type: MainScreenButton(WhatAButton::Exit),
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Exit",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });

            // GitHub button in the bottom-left corner
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center, // Центрирует по горизонтали
                                    align_items: AlignItems::Center, // Центрирует по вертикали
                                    ..default()
                                },
                                background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                                ..default()
                            },
                            ButtonType {
                                button_type: MainScreenButton(WhatAButton::Github),
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("icons/github.png")),
                                background_color: Color::srgb(0., 0., 0.).into(),
                                ..default()
                            });
                        });
                });
        });
}
