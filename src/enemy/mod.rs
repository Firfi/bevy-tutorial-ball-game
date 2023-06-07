use bevy::prelude::*;

mod systems;
mod components;
mod resources;
mod constants;
pub mod events;
mod utils;

use crate::enemy::systems::*;
use crate::enemy::events::*;
use crate::enemy::resources::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_enemies )
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(play_enemy_bounce_sound.after(update_enemy_direction))
            .add_system(enemy_player_collision.in_set(Labels::PlayerCollision))
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_enemy_after_time.after(tick_enemy_spawn_timer))
            .add_event::<EnemyBounceEvent>()
            .add_event::<EnemyPlayerCollideEvent>()
            .add_event::<EnemySpawnTimerEvent>()
            .init_resource::<EnemySpawnTimer>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Labels {
    PlayerCollision,
}
