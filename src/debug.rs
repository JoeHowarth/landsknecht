use crate::Player;
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
            app.register_inspectable::<Player>();
        }
    }
}
