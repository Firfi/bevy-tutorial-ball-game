use crate::player::types::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: PlayerName,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: PlayerName("Player".to_string()),
        }
    }
}