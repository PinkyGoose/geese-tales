//! This example illustrates the various features of Bevy UI.

mod game;
pub mod states;
mod ui;

use crate::game::toggle_pause_game;
use crate::states::{AppState, MainMenuState};
use crate::ui::main_menu::{despawn_ui, MyMainMenuPlugin};
use crate::ui::{activate_ui_camera, deactivate_ui_camera};
use bevy::log::LogPlugin;
use bevy::prelude::*;

fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .disable::<LogPlugin>(),
        )
        .add_plugins(MyMainMenuPlugin {
            state: AppState::MainMenu,
        })
        .insert_state(AppState::MainMenu);

    app.add_systems(OnEnter(AppState::MainMenu), (activate_ui_camera));
    app.add_systems(
        OnExit(AppState::MainMenu),
        (deactivate_ui_camera, despawn_ui),
    );
    app.add_systems(Update, toggle_pause_game);
    app.run();
}
