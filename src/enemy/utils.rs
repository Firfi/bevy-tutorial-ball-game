use bevy::prelude::*;
use crate::collision::detect_collision_circles;
use crate::enemy::components::*;
use crate::enemy::constants::*;
use crate::player::components::Player;
use rand::prelude::*;
use crate::player::constants::PLAYER_SIZE;

pub fn make_random_enemy(
    asset_server: &Res<AssetServer>,
    window: &Window,
    player_transform_query: &Query<&Transform, With<Player>>
) -> Option<(SpriteBundle, Enemy)> {
    let player_coordss: Vec<Vec3> = player_transform_query.iter().map(|player_transform| {
        player_transform.translation
    }).collect();
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    let transform = Transform::from_xyz(random_x, random_y, 0.0);
    let translation = transform.translation;
    const IMMEDIATE_DANGER_PAD: f32 = 200.0;
    // TODO retries
    let immediate_danger = player_coordss.into_iter().any(|player_coords| {
        detect_collision_circles((&translation, ENEMY_SIZE + IMMEDIATE_DANGER_PAD), (&player_coords, PLAYER_SIZE))
    });
    if immediate_danger {
        return None;
    }
    Some((
        SpriteBundle {
            transform,
            texture: asset_server.load("sprites/ball_red_large_alt.png"),
            ..default()
        },
        Enemy {
            direction: Vec2::new(random::<f32>(), random::<f32>()).try_normalize().unwrap_or_else(|| Vec2::ZERO)
        }
    ))
}