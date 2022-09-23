// #![windows_subsystem = "windows"]
use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    render::{camera::WindowOrigin, render_resource::FilterMode},
    window::{PresentMode, WindowId},
    winit::WinitWindows,
};
use bevy_inspector_egui::Inspectable;
use crossbeam_channel::{unbounded, Receiver};
use std::thread;
use rdev::listen;
use winit::window::Icon;

use tablet_tracker::{Config, RonLoader};
use debug::DebugPlugin;
use ui::UIPlugin;
mod ui;
mod debug;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 640.0;

#[derive(Deref)]
pub struct StreamReceiver(Receiver<rdev::Event>);

#[derive(Debug)]
struct StreamEvent(rdev::Event);

#[derive(Default, Component, Debug, Inspectable)]
pub struct Hand;

#[derive(Default)]
struct ConfHandle {
    handle: Handle<Config>,
}

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(WindowDescriptor {
            height: HEIGHT,
            width: HEIGHT * RESOLUTION,
            // position: WindowPosition::Centered(MonitorSelection::Current), // optional
            title: "Tablet tracker".into(), // optional
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_asset_loader::<RonLoader>()
        .init_resource::<ConfHandle>()
        .add_asset::<Config>()
        .add_event::<StreamEvent>()
        .insert_resource(FilterMode::Nearest)
        .insert_resource(ClearColor)
        // plugin for debugging the entities and components using "bevy-inspector-egui"
        .add_plugin(DebugPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, set_window_icon)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_startup_system(setup_events)
        .add_startup_system(load_image)
        .add_system(read_stream)
        .add_system(config)
        .add_system(image_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut conf: ResMut<ConfHandle>) {
    let config_handle: Handle<Config> = asset_server.load("config.ron");

    conf.handle = config_handle;

    let mut camera = Camera2dBundle::new_with_far(1.);
    camera.projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera);
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

fn load_image(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(Hand);
}

fn config(
    mut bg_color: ResMut<ClearColor>,
    mut hand_query: Query<(&Hand, &mut Sprite)>,
    conf_handle: Res<ConfHandle>,
    config_asset: Res<Assets<Config>>,
    mut events: EventReader<AssetEvent<Config>>,
) {
    if let Some(config) = config_asset.get(&conf_handle.handle) {
        for event in events.iter() {
            println!("{event:?}");
        }
        if let Ok((_hand, mut hand_sprite)) = hand_query.get_single_mut() {
            let mut hand_size = Vec2::splat(338.);

            if config.size.x > 0. {
                hand_size.x = config.size.x;
            }

            if config.size.y > 0. {
                hand_size.y = config.size.y;
            }

            hand_sprite.custom_size = Some(hand_size);
        }

        let bg = config.background.clone();
        if bg.to_ascii_lowercase() == "green".to_string() {
            bg_color.0 = Color::GREEN;
        } else if bg.to_ascii_lowercase() == "none".to_string() {
            bg_color.0 = Color::NONE;
        }
    } else {
        println!("config: no config");
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
    conf_handle: Res<ConfHandle>,
    config_asset: Res<Assets<Config>>,
) {
    for event in reader.iter() {
        match event.0.event_type {
            rdev::EventType::MouseMove {
                x: mouse_x,
                y: mouse_y,
            } => {
                if let Some(config) = config_asset.get(&conf_handle.handle) {
                    if let Ok((_hand, mut hand_transform)) = hand_query.get_single_mut() {
                        if let Some(window) = windows.get_primary() {
                            if let Some(position) = window.position() {
                                let scale_factor = window.scale_factor();
                                // subtract the position of the window from the mouse location
                                // to get the relative location
                                let x = (mouse_x as f32 - position.x as f32) + config.x_offset; // the more offset the more the images goes to the right
                                let y = (mouse_y as f32 - position.y as f32) - config.y_offset; // the more offset the more the images goes up
                                let z = Quat::from_rotation_z(
                                    (((y * 0.8) / window.height())
                                        - (((mouse_x as f32 - position.x as f32)
                                            - config.x_offset)
                                            * 0.6)
                                            / window.width())
                                        / scale_factor as f32,
                                );
                                // rotating the image to add hand like effect
                                hand_transform.rotation = z;
                                // move the image
                                hand_transform.translation.x = x / scale_factor as f32;
                                hand_transform.translation.y =
                                    window.height() - (y / scale_factor as f32);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
