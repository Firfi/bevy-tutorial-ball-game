use bevy::time::TimerMode::Repeating;
use bevy::prelude::*;
use crate::game::star::constants::*;

#[derive(Resource)]
pub struct StarSpawnTimer(pub Timer);

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer(Timer::from_seconds(STAR_SPAWN_TIME, Repeating))
    }
}