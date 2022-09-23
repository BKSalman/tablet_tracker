/// plugin for debugging the entities and components using "bevy-inspector-egui"


use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin, WorldInspectorParams};

use crate::Hand;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .insert_resource(WorldInspectorParams {
                        despawnable_entities: true,
                        highlight_changes: true,
                        ..Default::default()
                    })
                .add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Hand>();
        }
    }
}
