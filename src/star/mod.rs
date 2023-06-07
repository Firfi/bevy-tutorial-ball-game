mod systems;
mod components;
mod resources;
mod constants;
pub mod events;
mod utils;

use crate::star::systems::*;
use crate::star::events::*;
use crate::star::resources::*;

use bevy::prelude::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_stars )
            .add_system(star_player_collision.in_set(Labels::PlayerCollision))
            .add_system(play_star_player_collision_sound.after(star_player_collision))
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_star_after_time.after(tick_star_spawn_timer))
            .add_event::<StarCollideEvent>()
            .add_event::<StarSpawnTimerEvent>()
            .init_resource::<StarSpawnTimer>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Labels {
    PlayerCollision
}