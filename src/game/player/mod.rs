use bevy::prelude::*;
use crate::AppState;
use crate::game::enemy::Labels::PlayerCollision as EnemyCollision;
use crate::game::player::events::PlayerCollidedEnemyEvent;

mod systems;
pub mod components;
pub mod types;
pub mod constants;
mod utils;
pub mod events;

use crate::game::player::systems::*;
use crate::game::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((spawn_player, apply_system_buffers/*other systems want player entity here*/).chain().in_set(Labels::SpawnPlayer).in_schedule(OnEnter(AppState::Game)))
            .add_systems((player_movement, confine_player_movement.after(player_movement), player_enemy_collision_reaction.after(EnemyCollision))
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            )
            .add_event::<PlayerCollidedEnemyEvent>()
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Labels {
    SpawnPlayer,
}

