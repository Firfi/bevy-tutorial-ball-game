use bevy::prelude::*;
use crate::game::player::Labels as PlayerLabels;
use crate::AppState;

mod systems;
mod components;
mod resources;
mod constants;
pub mod events;
mod utils;

use crate::game::enemy::systems::*;
use crate::game::enemy::events::*;
use crate::game::enemy::resources::*;
use crate::game::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)).after(PlayerLabels::SpawnPlayer))
            .add_systems((
                enemy_movement,
                update_enemy_direction,
                play_enemy_bounce_sound.after(update_enemy_direction),
                enemy_player_collision.in_set(Labels::PlayerCollision),
                tick_enemy_spawn_timer,
                spawn_enemy_after_time.after(tick_enemy_spawn_timer)
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            )
            .add_event::<EnemyBounceEvent>()
            .add_event::<EnemyPlayerCollideEvent>()
            .add_event::<EnemySpawnTimerEvent>()
            .init_resource::<EnemySpawnTimer>()
        // exit state systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Labels {
    PlayerCollision,
}
