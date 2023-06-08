use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use crate::game::collision::detect_collision_circles;
use crate::game::enemy::components::*;
use crate::game::enemy::events::*;
use crate::game::enemy::constants::*;
use crate::game::enemy::utils::*;
use crate::game::enemy::resources::*;
use crate::game::player::components::Player;
use crate::timer::*;
use rand::prelude::*;
use crate::game::player::constants::PLAYER_SIZE;

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut eneme_query: Query<(&mut Enemy, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut event_writer: EventWriter<EnemyBounceEvent>,
) {
    let window = window_query.single();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    for (mut enemy, transform) in eneme_query.iter_mut() {
        let translation = transform.translation;
        let direction_diff_vector = Vec2::new(
            if (translation.x <= x_min && enemy.direction.x < 0.0) || (translation.x >= x_max && enemy.direction.x > 0.0) { -1.00 } else {1.00 },
            if (translation.y <= y_min && enemy.direction.y < 0.0) || (translation.y >= y_max && enemy.direction.y > 0.0) { -1.00 } else { 1.00 }
        );
        enemy.direction = enemy.direction * direction_diff_vector;
        if direction_diff_vector != Vec2::ONE {
            event_writer.send(EnemyBounceEvent);
        }
    }
}

pub fn play_enemy_bounce_sound(
    audio: Res<Audio>,
    mut event: EventReader<EnemyBounceEvent>,
    asset_server: Res<AssetServer>,
) {
    for _ in event.iter() {
        let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
        let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
        let sound_effect = if random::<f32>() > 0.5 { sound_effect_1 } else { sound_effect_2 };
        audio.play(sound_effect);
    }
}

pub fn enemy_player_collision(
    mut player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut event_writer: EventWriter<EnemyPlayerCollideEvent>,
) {
    for player_transform in player_query.iter_mut() {
        for enemy_transform in enemy_query.iter() {
            if detect_collision_circles(
                (&player_transform.translation, PLAYER_SIZE),
                (&enemy_transform.translation, ENEMY_SIZE),
            ) {
                event_writer.send(EnemyPlayerCollideEvent);
            }
        }
    }
}

pub fn tick_enemy_spawn_timer(
    mut timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    event_writer: EventWriter<EnemySpawnTimerEvent>,
) {
    spawn_timer(EnemySpawnTimerEvent, &mut timer.0, time, event_writer);
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    player_transform_query: Query<&Transform, With<Player>>
) {
    println!("Spawning enemies");
    let window = window_query.single();
    for _ in 0..NUMBER_OF_ENEMIES {
        if let Some(enemy) = make_random_enemy(&asset_server, window, &player_transform_query) {
            commands.spawn(enemy);
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}

pub fn spawn_enemy_after_time(
    mut commands: Commands,
    mut event_reader: EventReader<EnemySpawnTimerEvent>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_transform_query: Query<&Transform, With<Player>>,
) {
    for _ in event_reader.iter() {
        if let Some(enemy) = make_random_enemy(&asset_server, window_query.single(), &player_transform_query) {
            commands.spawn(enemy);
        }
    }
}