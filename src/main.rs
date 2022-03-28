#![allow(unused_imports, dead_code)]

use std::default::Default;

use bevy::{
    core::Zeroable,
    ecs::schedule::SystemDescriptor,
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    math::Vec2Swizzles,
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use heron::PhysicsPlugin;

use crate::{
    camera::CameraPlugin,
    debug::DebugPlugin,
    player::{Player, PlayerPlugin},
    sprites::SpritesPlugin,
    tilemap::TilemapPlugin,
};

mod camera;
mod debug;
mod player;
mod sprites;
mod tilemap;

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const ASPECT_RATIO: f32 = 16. / 9.;
const HEIGHT: f32 = 900.;
const CAMERA_SCALE: f32 = 100.;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * ASPECT_RATIO,
            height: HEIGHT,
            scale_factor_override: None,
            title: "Landsknecht".to_string(),
            vsync: true,
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LandsknechtPlugin)
        .run();
}

pub struct LandsknechtPlugin;

impl Plugin for LandsknechtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugPlugin);
        app.add_plugin(CameraPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(SpritesPlugin);
        app.add_plugin(TilemapPlugin);
        app.add_plugin(PhysicsPlugin::default());
        app.add_system_to_stage(CoreStage::PreUpdate, maintain_previous_transform);
    }
}

#[derive(heron::prelude::PhysicsLayer)]
pub enum Layer {
    Player,
    Obstacle,
    Unit,
    Bullet,
}

#[derive(Inspectable, Component)]
pub struct PrevTransform(pub Transform);

pub fn maintain_previous_transform(mut q: Query<(&Transform, &mut PrevTransform)>) {
    q.for_each_mut(|(trans, mut prev): (&Transform, Mut<PrevTransform>)| {
        prev.0 = trans.clone();
    })
}
