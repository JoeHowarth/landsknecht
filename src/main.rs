#![allow(unused_imports)]

use std::default::Default;

use bevy::{
    core::Zeroable,
    ecs::schedule::SystemDescriptor,
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    math::Vec2Swizzles,
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use player::PlayerPlugin;

use crate::{player::Player, sprites::SpritesPlugin};

mod player;
mod sprites;

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
        app.add_plugin(CameraPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(SpritesPlugin);
        app.add_plugin(DebugPlugin);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 100.0;
    camera.orthographic_projection.bottom = -100.0;
    camera.orthographic_projection.right = 100.0 * ASPECT_RATIO;
    camera.orthographic_projection.left = -100.0 * ASPECT_RATIO;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
    // Vec2::new(CAMERA_SCALE / HEIGHT * ASPECT_RATIO,  CAMERA_SCALE / HEIGHT);
}

pub struct CameraPlugin;
pub struct CameraToWorld(pub Vec2);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
        app.add_system(move_camera);
    }
}

fn move_camera(
    mut camera_q: Query<&mut Transform, With<Camera>>,
    mut mouse_movement: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let mut transform = camera_q.single_mut();
    for ev in scroll_evr.iter() {
        let ev: &MouseWheel = ev;
        dbg!(ev);
        let scale = match ev.unit {
            MouseScrollUnit::Pixel => 0.001,
            MouseScrollUnit::Line => 0.01,
        };
        let new = transform.scale + Vec3::new(ev.y, ev.y, 0.) * scale;
        transform.scale =
            new.clamp(Vec3::new(0.01, 0.01, 0.), Vec3::new(10., 10., 1500.));
    }

    if !mouse_buttons.any_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }
    const F: f32 = 2. * CAMERA_SCALE / HEIGHT;
    for ev in mouse_movement.iter() {
        transform.translation += Vec3::new(-ev.delta.x * F, ev.delta.y * F, 0.);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());
            app.register_inspectable::<Player>();
        }
    }
}
