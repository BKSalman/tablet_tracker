use bevy::{prelude::*, window::PresentMode, render::camera::WindowOrigin};
use enigo::Enigo;
use debug::DebugPlugin;
use bevy_inspector_egui::prelude::*;


mod debug;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 640.0;

#[derive(Default, Component, Debug, Inspectable)]
pub struct Hand {
    x_offset: f32,
    y_offset: f32,
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
    .add_startup_system(spawn_camera)
    .add_startup_system(load_image)
    .add_system(image_movement)
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
    let texture: Handle<Image> = asset_server.load("peepoAwesome.png");
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
        x_offset: 0.,
        y_offset: 32.
    });
}

fn image_movement(
    windows: Res<Windows>,
    mut hand_query: Query<(&Hand, &mut Transform)>,
) {
    // get the image
    let (hand, mut hand_transform) = hand_query.single_mut();
    // get the mouse location in a tuple
    // mouse.0 = x mouse.1 = y
    let mouse: (i32, i32) = Enigo::mouse_location();

    // get the window resource from bevy
    let window = windows.get_primary().unwrap();

    // check if available
    // then get the position of the window
    
    if let Some(position) = window.position() {
        // subtract the position of the window from the mouse location
        // to get the relative location

        let x = (mouse.0 - position.x) as f32 + hand.x_offset; // the more offset the more the images goes to the right
        let y = (mouse.1 - position.y) as f32 - hand.y_offset; // the more offset the more the images goes up
        
        // rotating the image to add hand like effect
        hand_transform.rotation = Quat::from_rotation_z((mouse.0 - position.x) as f32 / window.width());
        hand_transform.rotation = Quat::from_rotation_z((mouse.1 - position.y) as f32 / window.height());
        
        // move the image
        hand_transform.translation.x = x;
        hand_transform.translation.y = window.height() - y;
    } else {
        
    }
}