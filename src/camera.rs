use crate::{ASPECT_RATIO, CAMERA_SCALE, HEIGHT};
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::ScalingMode,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
        app.add_system(move_camera);
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
