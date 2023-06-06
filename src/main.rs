use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::time::TimerMode::Repeating;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct StarCount(pub usize);

#[derive(Debug, Clone)]
pub struct PlayerName(pub String);

impl StarCount {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: u32 = 4;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS: StarCount = StarCount(10);
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const ENEMY_SPAWN_TIME: f32 = 2.0;

pub struct EnemyBounceEvent;
pub struct EnemyPlayerCollideEvent;
pub struct StarSpawnTimerEvent;
pub struct EnemySpawnTimerEvent;
pub struct StarCollideEvent {
    pub who: Entity,
}
pub struct GameOverEvent {
    pub score: StarCount,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies )
        .add_startup_system(spawn_stars )
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(play_enemy_bounce_sound.after(update_enemy_direction))
        .add_system(enemy_player_collision)
        .add_system(player_enemy_collision_reaction.after(enemy_player_collision))
        .add_system(star_player_collision)
        .add_system(play_star_player_collision_sound.after(star_player_collision))
        .add_system(score_calculation.after(star_player_collision))
        .add_system(tick_star_spawn_timer)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_star_after_time.after(tick_star_spawn_timer))
        .add_system(spawn_enemy_after_time.after(tick_enemy_spawn_timer))
        .add_system(exit_game)
        .add_system(game_over_reaction.after(player_enemy_collision_reaction))
        .add_system(update_high_scores.before(game_over_reaction).after(player_enemy_collision_reaction))
        .add_event::<EnemyBounceEvent>()
        .add_event::<EnemyPlayerCollideEvent>()
        .add_event::<StarCollideEvent>()
        .add_event::<StarSpawnTimerEvent>()
        .add_event::<EnemySpawnTimerEvent>()
        .add_event::<GameOverEvent>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScores>()
        .run();
}

#[derive(Component)]
pub struct Player {
    name: PlayerName,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: PlayerName("Player".to_string()),
        }
    }
}

#[derive(Component, Debug)]
pub struct ScoreComponent(StarCount);

impl ScoreComponent {
    pub fn increment(&mut self) {
        self.0.increment();
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer(Timer);

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer(Timer::from_seconds(STAR_SPAWN_TIME, Repeating))
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<(PlayerName, StarCount)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: vec![] }
    }
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, Repeating))
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

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    player_transform_query: Query<&Transform, With<Player>>
) {
    let window = window_query.single();
    for _ in 0..NUMBER_OF_ENEMIES {
        if let Some(enemy) = make_random_enemy(&asset_server, window, &player_transform_query) {
            commands.spawn(enemy);
        }
    }
}

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

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}



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

pub fn play_enemy_player_collision_sound(
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
) {
    let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
    audio.play(sound_effect);
}

pub fn player_enemy_collision_reaction(
    audio: Res<Audio>,
    mut event: EventReader<EnemyPlayerCollideEvent>,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<GameOverEvent>,
    score_query: Query<&ScoreComponent, With<Player>>,
) {
    for _ in event.iter() {
        play_enemy_player_collision_sound(&audio, &asset_server);
        let score = score_query.single().0.clone();
        // TODO add player data here so we don't have to bother with system ordering
        event_writer.send(GameOverEvent { score })
    }

}

pub fn game_over_reaction(
    mut event: EventReader<GameOverEvent>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for event in event.iter() {
        println!("despawning");
        player_query.for_each(|player_entity|
            commands.entity(player_entity).despawn()
        );
        println!("Game Over! Score: {:?}", event.score);
    }
}

pub fn detect_collision_circles(
    (translation_a, size_a): (&Vec3, f32),
    (translation_b, size_b): (&Vec3, f32),
) -> bool {
    // TODO glam why we clone???
    let distance = translation_a.distance(translation_b.clone());
    distance < size_a / 2.0 + size_b / 2.0
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

pub fn spawn_timer<T: Event>(
    event: T,
    timer: &mut Timer,
    time: Res<Time>,
    mut event_writer: EventWriter<T>,
) {
    timer.tick(time.delta());
    if timer.finished() {
        event_writer.send(event);
        timer.reset();
    }
}

pub fn tick_star_spawn_timer(
    mut timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
    event_writer: EventWriter<StarSpawnTimerEvent>,
) {
    spawn_timer(StarSpawnTimerEvent, &mut timer.0, time, event_writer);
}

pub fn tick_enemy_spawn_timer(
    mut timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    event_writer: EventWriter<EnemySpawnTimerEvent>,
) {
    spawn_timer(EnemySpawnTimerEvent, &mut timer.0, time, event_writer);
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_event_writer.send(AppExit);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut high_scores: ResMut<HighScores>,
    player_query: Query<&Player>,
) {
    for event in game_over_event_reader.iter() {
        println!("updating scores");
        // TODO player name
        let player = player_query.single();
        high_scores.scores.push((PlayerName(player.name.0.to_string()), event.score.clone()));
    }
}