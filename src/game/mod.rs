mod systems;
pub mod events;

use bevy::prelude::*;
use crate::enemy::Labels::PlayerCollision as PlayerEnemyCollision;

use crate::game::systems::*;
use crate::game::events::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
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