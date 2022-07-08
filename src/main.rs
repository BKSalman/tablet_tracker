use bevy::{prelude::*, window::PresentMode, render::camera::WindowOrigin};
use enigo::Enigo;
use debug::DebugPlugin;
use bevy_inspector_egui::prelude::*;


mod debug;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 640.0;

#[derive(Component, Debug, Inspectable)]
pub struct Hand;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        height: HEIGHT,
        width: HEIGHT * RESOLUTION,
        position: Some(Vec2::new(400.0, 200.0)),
        title: "Letters".into(),
        present_mode: PresentMode::Fifo,
        #[cfg(target_arch = "wasm32")]
        canvas: Some("#bevy-canvas".to_string()),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugPlugin)
    .add_startup_system(spawn_camera)
    .add_startup_system(load_image)
    .add_system(mouse)
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
    .insert(Hand);
}

fn mouse(
    windows: Res<Windows>,
    mut hand_query: Query<&mut Transform, With<Hand>>,
) {
    let mut hand_transform = hand_query.single_mut();
    let mouse: (i32, i32) = Enigo::mouse_location();
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.position() {
        let x = mouse.0 - position.x;
        let y = mouse.1 - position.y;
        hand_transform.translation.x = x as f32;
        hand_transform.translation.y = window.height() - y as f32;
    } else {
        
    }
}