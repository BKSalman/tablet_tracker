use bevy::{prelude::*, asset::{AssetLoader, LoadContext, LoadedAsset}, reflect::TypeUuid, utils::BoxedFuture};
use serde::Deserialize;

#[derive(Default, Component, Debug, Deserialize, TypeUuid)]
#[ uuid = "9b511981-28ac-4888-8261-f94d6fb19b25" ]
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

    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let loaded = ron::de::from_bytes::<Config>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(loaded));
            Ok(())
        })
    }
}
