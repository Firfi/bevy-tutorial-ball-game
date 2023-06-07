mod systems;
pub mod components;
mod resources;
pub mod types;

use crate::enemy::Labels::PlayerCollision as PlayerEnemyCollision;
use crate::game::Labels::GameOver as GameOverLabel;
use crate::star::Labels::PlayerCollision as StarPlayerCollision;

use crate::score::systems::*;
use crate::score::resources::*;

use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(score_calculation.after(StarPlayerCollision))
            .add_system(update_high_scores.before(GameOverLabel).after(PlayerEnemyCollision))
            .init_resource::<HighScores>();
    }
}