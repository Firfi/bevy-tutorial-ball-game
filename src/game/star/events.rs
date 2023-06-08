use bevy::prelude::*;

pub struct StarSpawnTimerEvent;
pub struct StarCollideEvent {
    pub who: Entity,
}