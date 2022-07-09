use bevy::{prelude::*, window::{PresentMode, WindowId}, render::camera::WindowOrigin, winit::WinitWindows};
use enigo::Enigo;
use debug::DebugPlugin;
use bevy_inspector_egui::prelude::*;
use winit::window::Icon;

mod debug;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 640.0;

#[derive(Default, Component, Debug, Inspectable)]
pub struct Hand {
    x_offset: f32,
    y_offset: f32,
    size: Vec2,
}

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        height: HEIGHT,
        width: HEIGHT * RESOLUTION,
        position: Some(Vec2::new(400.0, 200.0)), // optional
        title: "Tablet tracker".into(), // optional
        present_mode: PresentMode::Fifo,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::NONE)) // transparent background
    .add_plugins(DefaultPlugins)
    
    // plugin for debugging the entities and components using "bevy-inspector-egui"
    .add_plugin(DebugPlugin) 
    .add_startup_system(set_window_icon)
    .add_startup_system(spawn_camera)
    .add_startup_system(load_image)
    .add_system(image_movement)
    .add_system(image_size)
    .run();
}

fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera);
}

fn load_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("Hand.png");

    let sprite = Sprite{
        anchor: bevy::sprite::Anchor::TopLeft,
        ..Default::default()
    };

    commands.spawn_bundle(SpriteBundle{
        sprite,
        texture,
        ..Default::default()
    })
    .insert(Name::new("Hand"))
    .insert(Transform::from_xyz(0., 0., 0.))
    .insert(Hand{
        x_offset: -13.,
        y_offset: 35.,
        ..Default::default()
    });
}

fn image_size(
    mut hand_query: Query<(&Hand, &mut Sprite)>,
) {
    if let Ok(mut hand) = hand_query.get_single_mut() {
        if hand.0.size.x > 0. && hand.0.size.y > 0. {
            hand.1.custom_size = Some(hand.0.size);
        }
    } else {

    }
}

fn image_movement(
    windows: Res<Windows>,
    mut hand_query: Query<(&Hand, &mut Transform)>,
) {
    // get the window resource from bevy
    let window = windows.get_primary();
    // get the mouse location in a tuple
    // mouse.0 = x mouse.1 = y
    let mouse: (i32, i32) = Enigo::mouse_location();
    
    // get the image
    // hand.0 = Hand, hand.1 Transform
    if let Ok(mut hand) = hand_query.get_single_mut() {
        match window {
            Some(window) => {
                // check if available
                // then get the position of the window
                if let Some(position) = window.position() {
                    // subtract the position of the window from the mouse location
                    // to get the relative location
            
                    let x = (mouse.0 - position.x) as f32 + hand.0.x_offset; // the more offset the more the images goes to the right
                    let y = (mouse.1 - position.y) as f32 - hand.0.y_offset; // the more offset the more the images goes up
                    
                    // rotating the image to add hand like effect
                    hand.1.rotation = Quat::from_rotation_z( 
                        ((y * 0.8) / window.height()) - (((mouse.0 - position.x) as f32 - hand.0.x_offset) * 0.7) / window.width()
                    );
            
                    // move the image
                    hand.1.translation.x = x;
                    hand.1.translation.y = window.height() - y;
                } else {
                    
                }
            },
            None =>{}
        }
    }

}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
) {
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