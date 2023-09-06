use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
    window::WindowId,
    winit::WinitWindows,
};
use serde::{Deserialize, Serialize};
use winit::window::Icon;

#[derive(Default, Component, Debug, Deserialize, Serialize, TypeUuid)]
#[uuid = "9b511981-28ac-4888-8261-f94d6fb19b25"]
pub struct Config {
    pub x_offset: f32,
    pub y_offset: f32,
    pub size: Vec2,
    pub background: String,
}

#[derive(Default)]
pub struct RonLoader {
    pub extensions: Vec<&'static str>,
    pub _t: Config,
}

impl AssetLoader for RonLoader {
    fn extensions(&self) -> &[&str] {
        &["ron"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let loaded = ron::de::from_bytes::<Config>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(loaded));
            Ok(())
        })
    }
}

pub fn set_window_icon(windows: NonSend<WinitWindows>) {
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
