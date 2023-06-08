use bevy::prelude::*;
use crate::game::star::components::*;
use rand::prelude::*;

pub fn make_random_star(
    asset_server: &Res<AssetServer>,
    window: &Window,
) -> (SpriteBundle, Star) {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    (
        SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {}
    )
}