use bevy::prelude::*;
use crate::score::types::*;

#[derive(Component, Debug)]
pub struct ScoreComponent(pub StarCount);

impl ScoreComponent {
    pub fn increment(&mut self) {
        self.0.increment();
    }
}