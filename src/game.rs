use crate::states::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{in_state, IntoSystemConfigs, KeyCode, NextState, Res, ResMut, State, States};

pub struct MyGamePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MyGamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_pause_game.run_if(in_state(self.state.clone())),
        );
    }
}
//TODO переместить эту систему в гейм плагин и менять стейт на Pause
pub fn toggle_pause_game(
    state: Res<State<AppState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // if let Some()
    if input.just_pressed(KeyCode::Escape) {
        match state.get() {
            AppState::Game => next_state.set(AppState::MainMenu),
            AppState::MainMenu => {
                info!("переходим в ingame");
                next_state.set(AppState::Game)
            }
            _ => {}
        }
    }
}
