use crate::score::types::*;
use bevy::prelude::*;
use crate::player::types::*;

#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<(PlayerName, StarCount)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: vec![] }
    }
}

