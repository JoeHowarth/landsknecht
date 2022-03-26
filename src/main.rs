#![allow(unused_imports)]

use bevy::ecs::schedule::SystemDescriptor;
use std::default::Default;

use bevy::{prelude::*, render::camera::ScalingMode};

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const ASPECT_RATIO: f32 = 16. / 9.;
const HEIGHT: f32 = 900.;

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
        app.add_startup_system(spawn_camera);
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            load_sprite_sheet,
        );
    }
}

fn load_sprite_sheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image =
        assets.load("kenney_rts_sci_fi/Spritesheet/scifiRTS_spritesheet.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Default::default(),
        0,
        0,
        Default::default(),
    );
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * ASPECT_RATIO;
    camera.orthographic_projection.left = -1.0 * ASPECT_RATIO;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
