mod systems;

use bevy::prelude::*;
use crate::camera::systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera);
    }
}