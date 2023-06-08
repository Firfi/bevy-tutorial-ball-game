use bevy::app::AppExit;
use bevy::prelude::*;
use crate::AppState;
use crate::game::enemy::events::EnemyPlayerCollideEvent;
use crate::game::over::events::*;
use crate::game::player::components::Player;
use crate::game::score::components::ScoreComponent;

pub fn game_over_reaction(
    mut event: EventReader<GameOverEvent>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in event.iter() {
        player_query.for_each(|player_entity|
            commands.entity(player_entity).despawn()
        );
        println!("Game Over! Score: {:?}", event.score);
        next_app_state.set(AppState::GameOver);
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_event_writer.send(AppExit);
    }
}

pub fn player_enemy_collision_reaction(
    mut event_writer: EventWriter<GameOverEvent>,
    score_query: Query<&ScoreComponent, With<Player>>,
    mut event: EventReader<EnemyPlayerCollideEvent>,
) {
    for _event in event.iter() {
        let score = score_query.single().0.clone();
        // TODO add player data here so we don't have to bother with system ordering
        event_writer.send(GameOverEvent { score })
    }
}