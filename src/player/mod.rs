use bevy::prelude::*;
use crate::enemy::Labels::PlayerCollision as EnemyCollision;
use crate::player::events::PlayerCollidedEnemyEvent;

mod systems;
pub mod components;
pub mod types;
pub mod constants;
mod utils;
pub mod events;

use crate::player::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(player_enemy_collision_reaction.after(EnemyCollision))
            .add_event::<PlayerCollidedEnemyEvent>();
    }
}
