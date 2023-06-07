use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use crate::enemy::events::EnemyPlayerCollideEvent;
use crate::player::components::*;
use crate::player::constants::*;
use crate::player::utils::play_enemy_player_collision_sound;
use crate::score::components::ScoreComponent;
use crate::score::types::StarCount;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        direction = direction.try_normalize().unwrap_or_else(|| direction);

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    for mut transform in player_query.iter_mut() {
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        }
        if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;

    }
}

pub fn player_enemy_collision_reaction(
    audio: Res<Audio>,
    mut event: EventReader<EnemyPlayerCollideEvent>,
    asset_server: Res<AssetServer>
) {
    for _ in event.iter() {
        play_enemy_player_collision_sound(&audio, &asset_server);
    }

}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large_alt.png"),
                ..default()
            },
            Player { ..default() },
            ScoreComponent(StarCount(0)),
        )
    );
}