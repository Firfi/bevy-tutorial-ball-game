use bevy::prelude::*;

mod game;
mod player;
mod camera;
mod enemy;
mod star;
mod score;
mod timer;
mod collision;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(game::GamePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(star::StarPlugin)
        .add_plugin(score::ScorePlugin)
        .run();
}



