use bevy::prelude::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::{BUTTON_STYLE, get_button_text_style, get_title_text_style, IMAGE_STYLE, MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR, TITLE_STYLE};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {

            }
        )
    ).with_children(|parent| {
        // title
        parent.spawn((
            NodeBundle {
                style: TITLE_STYLE,
                ..default()
            }
        )).with_children(|parent| {
            // image 1
            parent.spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load("sprites/ball_blue_large_alt.png").into(),
                ..default()
            });
            // text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Bevy Ball Game".to_string(),
                            style: get_title_text_style(&asset_server),
                            ..default()
                        }
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            // image 2
            parent.spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load("sprites/ball_red_large_alt.png").into(),
                ..default()
            });
        });
        // play
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            PlayButton {}
        )).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Play".to_string(),
                            style: get_button_text_style(&asset_server),
                        }
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
        // quit
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            QuitButton {}
        )).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Quit".to_string(),
                            style: get_button_text_style(&asset_server),
                        }
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });

    }).id();
    main_menu_entity
}