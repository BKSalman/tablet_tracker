// #![windows_subsystem = "windows"]
use bevy::{
    prelude::*,
    render::{camera::WindowOrigin, render_resource::FilterMode},
    window::{PresentMode, WindowId},
    winit::WinitWindows,
};
use crossbeam_channel::{unbounded, Receiver};
use std::{fs, thread};
// use debug::DebugPlugin;
// use bevy_inspector_egui::prelude::*;
use rdev::listen;
use ron::de::from_str;
use serde::Deserialize;
use winit::window::Icon;

// mod debug;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 640.0;

#[derive(Default, Component, Debug, Deserialize)]
struct Config {
    x_offset: f32,
    y_offset: f32,
    size: Vec2,
    background: String,
}

#[derive(Deref)]
pub struct StreamReceiver(Receiver<rdev::Event>);

#[derive(Debug)]
struct StreamEvent(rdev::Event);

#[derive(Default, Component, Debug)]
pub struct Hand {
    config: Config,
}

fn main() {
    let config_file = fs::read_to_string("assets/config.ron").unwrap();

    let config: Config = from_str(&config_file).unwrap_or_else(|e| {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    App::new()
        .insert_resource(WindowDescriptor {
            height: HEIGHT,
            width: HEIGHT * RESOLUTION,
            // position: WindowPosition::Centered(MonitorSelection::Current), // optional
            title: "Tablet tracker".into(),                                // optional
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_event::<StreamEvent>()
        .insert_resource(FilterMode::Nearest)
        .insert_resource(config)
        .insert_resource(ClearColor)
        .add_plugins(DefaultPlugins)
        // plugin for debugging the entities and components using "bevy-inspector-egui"
        // .add_plugin(DebugPlugin)
        .add_startup_system(set_window_icon)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_image)
        .add_startup_system(image_size)
        .add_startup_system(setup_events)
        .add_system(read_stream)
        .add_system(background_color)
        .add_system(image_movement)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::new_with_far(1.);
    camera.projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera);
}

fn background_color(mut bg_color: ResMut<ClearColor>, config: Res<Config>) {
    let bg = config.background.clone();
    if bg.to_ascii_lowercase() == "green".to_string() {
        bg_color.0 = Color::GREEN;
    } else if bg.to_ascii_lowercase() == "none".to_string() {
        bg_color.0 = Color::NONE;
    }
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/bksalmSalute.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

fn load_image(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
    let texture: Handle<Image> = asset_server.load("Hand.png");

    let sprite = Sprite {
        anchor: bevy::sprite::Anchor::TopLeft,
        ..Default::default()
    };

    commands
        .spawn_bundle(SpriteBundle {
            sprite,
            texture,
            ..Default::default()
        })
        .insert(Name::new("Hand"))
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(Hand {
            config: Config {
                x_offset: config.x_offset,
                y_offset: config.y_offset,
                size: config.size,
                ..Default::default()
            },
        });
}

fn image_size(mut hand_query: Query<(&Hand, &mut Sprite)>) {
    if let Ok(mut hand) = hand_query.get_single_mut() {
        if hand.0.config.size.x > 0. && hand.0.config.size.y > 0. {
            hand.1.custom_size = Some(hand.0.config.size);
        }
    } else {
    }
}

fn setup_events(mut commands: Commands) {
    let (sender, reciever) = unbounded::<rdev::Event>();
    thread::spawn(move || {
        listen(move |event| sender.send(event).expect("Failed to send event"))
            .expect("Could not listen");
    });
    commands.insert_resource(StreamReceiver(reciever));
}

fn read_stream(receiver: ResMut<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
    for from_stream in receiver.try_iter() {
        events.send(StreamEvent(from_stream));
    }
}

fn image_movement(
    windows: Res<Windows>,
    mut hand_query: Query<(&Hand, &mut Transform)>,
    mut reader: EventReader<StreamEvent>,
) {
    for event in reader.iter() {
        match event.0.event_type {
            rdev::EventType::MouseMove {
                x: mouse_x,
                y: mouse_y,
            } => {
                if let Ok((hand, mut hand_transform)) = hand_query.get_single_mut() {
                    if let Some(window) = windows.get_primary() {
                        if let Some(position) = window.position() {
                            // subtract the position of the window from the mouse location
                            // to get the relative location
                            let x = (mouse_x as f32 - position.x as f32) + hand.config.x_offset; // the more offset the more the images goes to the right
                            let y = (mouse_y as f32 - position.y as f32) - hand.config.y_offset; // the more offset the more the images goes up

                            // rotating the image to add hand like effect
                            hand_transform.rotation = Quat::from_rotation_z(
                                ((y * 0.8) / window.height()) - (((mouse_x as f32 - position.x as f32) - hand.config.x_offset) * 0.6) / window.width()
                            );

                            // move the image
                            hand_transform.translation.x = x;
                            hand_transform.translation.y = window.height() - y;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
