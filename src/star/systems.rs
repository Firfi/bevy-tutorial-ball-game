use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use crate::player::components::Player;
use crate::player::constants::PLAYER_SIZE;
use crate::star::events::*;
use crate::star::components::*;
use crate::star::constants::*;
use crate::star::utils::*;
use crate::star::resources::*;
use crate::timer::spawn_timer;

pub fn play_star_player_collision_sound(
    audio: Res<Audio>,
    mut event: EventReader<StarCollideEvent>,
    asset_server: Res<AssetServer>,
) {
    for _ in event.iter() {
        let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
        audio.play(sound_effect);
    }
}

// TODO reuse the collision module
pub fn star_player_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut event_writer: EventWriter<StarCollideEvent>,
) {
    for (player_entity, player_transform) in player_query.iter_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                event_writer.send(StarCollideEvent { who: player_entity });
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn tick_star_spawn_timer(
    mut timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
    event_writer: EventWriter<StarSpawnTimerEvent>,
) {
    spawn_timer(StarSpawnTimerEvent, &mut timer.0, time, event_writer);
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    for _ in 0..NUMBER_OF_STARS.0 {
        commands.spawn(
            make_random_star(&asset_server, window)
        );
    }
}

pub fn spawn_star_after_time(
    mut commands: Commands,
    mut event_reader: EventReader<StarSpawnTimerEvent>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for _ in event_reader.iter() {
        commands.spawn(make_random_star(&asset_server, window_query.single()));
    }
}