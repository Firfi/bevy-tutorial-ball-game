use bevy::app::Plugin;
use bevy::prelude::*;

mod over;
mod enemy;
mod player;
mod score;
mod star;
mod collision;
mod systems;

use systems::*;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_state::<SimulationState>()
            .add_plugin(over::GameOverPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(enemy::EnemyPlugin)
            .add_plugin(star::StarPlugin)
            .add_plugin(score::ScorePlugin)
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Copy, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

