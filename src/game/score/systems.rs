use bevy::prelude::*;
use crate::game::over::events::GameOverEvent;
use crate::game::player::components::Player;
use crate::game::player::types::PlayerName;
use crate::game::score::components::*;
use crate::game::score::resources::*;
use crate::game::star::events::StarCollideEvent;

pub fn score_calculation(
    mut entity_score_query: Query<(Entity, &mut ScoreComponent)>,
    mut event_reader: EventReader<StarCollideEvent>,
) {
    for event in event_reader.iter() {
        for (entity, mut score) in entity_score_query.iter_mut() {
            if entity == event.who {
                score.increment();
                println!("Score: {:?}", score.0);
            }
        }
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut high_scores: ResMut<HighScores>,
    player_query: Query<&Player>,
) {
    // TODO call it with score_calculation why not
    for event in game_over_event_reader.iter() {
        println!("updating scores");
        // TODO player name
        let player = player_query.single();
        high_scores.scores.push((PlayerName(player.name.0.to_string()), event.score.clone()));
    }
}