use bevy::prelude::*;
use crate::systems::{transition_to_game_state, transition_to_main_menu_state};

mod game;
mod camera;
mod timer;
mod main_menu;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(game::GamePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_game_state)
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Copy, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}