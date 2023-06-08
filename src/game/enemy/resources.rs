use bevy::time::TimerMode::Repeating;
use crate::game::enemy::constants::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, Repeating))
    }
}