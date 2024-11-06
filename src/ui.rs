pub mod main_menu;

use bevy::prelude::*;

pub fn activate_ui_camera(mut camera: Query<&mut Camera, With<IsDefaultUiCamera>>) {
    for mut i in camera.iter_mut() {
        i.is_active = true;
    }
}

pub fn deactivate_ui_camera(mut camera: Query<&mut Camera, With<IsDefaultUiCamera>>) {
    for mut i in camera.iter_mut() {
        i.is_active = false;
    }
}

pub fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}
