use bevy::app::AppExit;
use bevy::prelude::*;
use crate::AppState;
use crate::main_menu::components::*;
use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};

pub fn interact_button_color<T: Component>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<T>)>,
    mut cb: impl FnMut() -> ()
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                background_color.0 = PRESSED_BUTTON_COLOR.into();
                cb();
            },
            Interaction::Hovered => {
                background_color.0 = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                background_color.0 = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}

pub fn interact_play_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_button_color(button_query, || {
        app_state_next_state.set(AppState::Game);
    });
}

pub fn interact_quit_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    interact_button_color(button_query, || {
        exit_event_writer.send(AppExit);
    });
}