mod systems;
mod components;
mod styles;

use systems::layout::*;

use bevy::prelude::*;
use crate::AppState;
use crate::main_menu::systems::interactions::{interact_play_button, interact_quit_button};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems((interact_play_button, interact_quit_button).in_set(OnUpdate(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}