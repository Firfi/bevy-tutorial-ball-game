mod systems;
mod components;
mod resources;
mod constants;
pub mod events;
mod utils;

use crate::game::star::systems::*;
use crate::game::star::events::*;
use crate::game::star::resources::*;

use bevy::prelude::*;
use crate::AppState;
use crate::game::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            .add_systems((
                             star_player_collision.in_set(Labels::PlayerCollision),
                         play_star_player_collision_sound.after(star_player_collision),
                             tick_star_spawn_timer,
                             spawn_star_after_time.after(tick_star_spawn_timer)
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            )
            .add_event::<StarCollideEvent>()
            .add_event::<StarSpawnTimerEvent>()
            .init_resource::<StarSpawnTimer>()
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));

    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Labels {
    PlayerCollision
}