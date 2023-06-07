use bevy::prelude::*;

pub fn play_enemy_player_collision_sound(
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
) {
    let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
    audio.play(sound_effect);
}