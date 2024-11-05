//! This example illustrates the various features of Bevy UI.

mod states;
mod ui;

use crate::ui::setup_ui;
use bevy::{
    prelude::*,
};
use bevy::log::LogPlugin;
use crate::states::MyAppState;
use crate::ui::{despawn_ui, MyUiPlugin};

fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .disable::<LogPlugin>(),
        )
        .add_plugins(MyUiPlugin {
            state: MyAppState::MainMenu
        })
        .insert_state(MyAppState::MainMenu);

    app.add_systems(OnEnter(MyAppState::MainMenu), setup_ui);
    app.add_systems(OnExit(MyAppState::MainMenu), despawn_ui);

    app.run();
}
