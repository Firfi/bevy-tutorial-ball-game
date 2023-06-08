mod systems;
pub mod events;

use bevy::prelude::*;
use crate::game::enemy::Labels::PlayerCollision as PlayerEnemyCollision;

use crate::game::over::systems::*;
use crate::game::over::events::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(exit_game)
            .add_system(player_enemy_collision_reaction.after(PlayerEnemyCollision))
            .add_system(game_over_reaction.in_set(Labels::GameOver).after(player_enemy_collision_reaction))
            .add_event::<GameOverEvent>();

    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Labels {
    GameOver,
}