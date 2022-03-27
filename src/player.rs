use crate::{debug::ENABLE_INSPECTOR, sprites::SpriteSheet};
use bevy::{core::Zeroable, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub const TILE_SIZE: f32 = 10.;

#[derive()]
pub struct Collidable;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        app.add_system(move_player);

        if ENABLE_INSPECTOR {
            app.register_inspectable::<Player>();
        }
    }
}

#[derive(Inspectable, Component)]
pub struct Player {
    pub speed: f32,
}

fn spawn_player(mut commands: Commands, sprites: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(27);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: sprites.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 10.0 })
        .insert(Name::new("Player"))
        .id();
}

fn move_player(
    mut player_q: Query<(&Player, &mut Transform)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let pressed = input.get_pressed();
    if pressed.len() == 0 {
        return;
    }
    let (player, mut trans) = player_q.single_mut();
    let mut dir = Vec3::zeroed();
    for key in pressed {
        let key: KeyCode = *key;
        match key {
            KeyCode::W | KeyCode::Up => {
                dir.y += 1.;
            }
            KeyCode::S | KeyCode::Down => {
                dir.y -= 1.;
            }
            KeyCode::D | KeyCode::Right => {
                dir.x += 1.;
            }
            KeyCode::A | KeyCode::Left => {
                dir.x -= 1.;
            }
            _ => {}
        }
    }
    if dir == Vec3::ZERO {
        return;
    }
    trans.translation += 0.5
        * TILE_SIZE
        * player.speed
        * time.delta().as_secs_f32()
        * dir.normalize();
}
