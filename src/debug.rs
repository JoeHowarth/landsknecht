use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use crate::Player;

pub struct DebugPlugin;

pub const ENABLE_INSPECTOR: bool = cfg!(debug_assertions);

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if ENABLE_INSPECTOR {
            app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
        }
    }
}
