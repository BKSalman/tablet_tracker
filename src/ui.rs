use bevy::prelude::*;
use ron::ser::PrettyConfig;
use tablet_tracker::Config;

use crate::ConfHandle;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(toggle_menu)
            .add_system(display_values)
            .add_system(button_system);
    }
}

#[derive(Component)]
struct MenuState {
    is_opened: bool,
}

#[derive(Component)]
struct XValue;

#[derive(Component)]
struct YValue;

#[derive(Component)]
struct Container;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const INCREASEX: &str = "+x";
const INCREASEY: &str = "+y";

const DECREASEX: &str = "-x";
const DECREASEY: &str = "-y";

const SAVE_TO_FILE: &str = "Save to file";

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    conf_handle: ResMut<ConfHandle>,
    mut config_asset: ResMut<Assets<Config>>,
    text_query: Query<&Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if let Some(config) = config_asset.get_mut(&conf_handle.handle) {
                    if text.sections[0].value == INCREASEX {
                        config.x_offset += 1.;
                    } else if text.sections[0].value == DECREASEX {
                        config.x_offset -= 1.;
                    } else if text.sections[0].value == INCREASEY {
                        config.y_offset += 1.;
                    } else if text.sections[0].value == DECREASEY {
                        config.y_offset -= 1.;
                    } else if text.sections[0].value == SAVE_TO_FILE {
                        let ron_options = ron::options::Options::default();
                        let confing_str = ron_options
                            .to_string_pretty(config, PrettyConfig::default())
                            .unwrap();

                        std::fs::write(
                            std::env::current_dir().unwrap().join("assets/config.ron"),
                            confing_str,
                        )
                        .unwrap();
                    }
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                display: Display::None,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Container)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ))
                .insert(XValue);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        INCREASEX,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        DECREASEX,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn_bundle(TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ))
                .insert(YValue);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        INCREASEY,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        DECREASEY,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        SAVE_TO_FILE,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn toggle_menu(keyboard: Res<Input<KeyCode>>, mut node_query: Query<&mut Style, With<Container>>) {
    for mut node in node_query.iter_mut() {
        if keyboard.just_pressed(KeyCode::Escape) {
            if node.display == Display::Flex {
                node.display = Display::None;
            } else if node.display == Display::None {
                node.display = Display::Flex;
            }
        }
    }
}

fn display_values(
    mut x_query: Query<&mut Text, With<XValue>>,
    mut y_query: Query<&mut Text, (With<YValue>, Without<XValue>)>,
    conf_handle: Res<ConfHandle>,
    config_asset: Res<Assets<Config>>,
) {
    let config = config_asset.get(&conf_handle.handle);
    let y_offset = config.map(|c| c.y_offset).unwrap_or(0.);
    let x_offset = config.map(|c| c.x_offset).unwrap_or(0.);
    for mut text in x_query.iter_mut() {
        text.sections[0].value = x_offset.to_string();
    }
    for mut text in y_query.iter_mut() {
        text.sections[0].value = y_offset.to_string();
    }
}
