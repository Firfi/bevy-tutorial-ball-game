use bevy::prelude::*;
use crate::AppState;
use crate::game::SimulationState;

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    let target_state = AppState::Game;
    if app_state.0 == target_state {
        return;
    }
    if !keyboard_input.just_pressed(KeyCode::G) {
        return
    }
    println!("Transition to game");
    next_app_state.set(target_state);
}
pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    let target_state = AppState::MainMenu;
    if app_state.0 == target_state {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::M) {
        println!("Transition to main menu");
        next_app_state.set(target_state);
        next_simulation_state.set(SimulationState::Paused);
    }
}