use std::fs;

use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};

use crate::{
    player::TILE_SIZE,
    sprites::{spawn_ascii, AsciiSheet},
    Layer, CAMERA_SCALE,
};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_map);
    }
}

fn load_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let map_file = std::fs::read_to_string("assets/map.txt").unwrap();
    let width = map_file.lines().next().unwrap().chars().count();
    let height = map_file.lines().count();

    let mut tiles = Vec::with_capacity(width * height);
    let offset = Vec2::new(
        -(width as f32) / 2. * TILE_SIZE,
        height as f32 / 2. * TILE_SIZE,
    );
    for (row, line) in map_file.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let tile = spawn_ascii(
                &mut commands,
                &ascii,
                c as u8,
                Vec2::new(col as f32 * TILE_SIZE, row as f32 * -TILE_SIZE) + offset,
            );
            match c {
                '#' => {
                    commands
                        .entity(tile)
                        .insert(RigidBody::Static)
                        .insert(CollisionShape::Cuboid {
                            half_extends:  Vec3::splat(TILE_SIZE / 2.),
                            border_radius: None,
                        })
                        .insert(
                            CollisionLayers::all_masks::<Layer>()
                                .with_group(Layer::Obstacle),
                        );
                }
                _ => {}
            };
            tiles.push(tile);
        }
    }
    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}
