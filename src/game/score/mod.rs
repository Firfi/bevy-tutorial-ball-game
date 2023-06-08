mod systems;
pub mod components;
mod resources;
pub mod types;

use crate::game::enemy::Labels::PlayerCollision as PlayerEnemyCollision;
use crate::game::over::Labels::GameOver as GameOverLabel;
use crate::game::star::Labels::PlayerCollision as StarPlayerCollision;

use crate::game::score::systems::*;
use crate::game::score::resources::*;

use bevy::prelude::*;
use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(score_calculation.after(StarPlayerCollision).run_if(in_state(AppState::Game)))
            .add_system(update_high_scores.before(GameOverLabel).after(PlayerEnemyCollision))
            .init_resource::<HighScores>();
    }
}