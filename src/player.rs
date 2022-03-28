use bevy::{
    core::Zeroable,
    math::Vec3Swizzles,
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use heron::{
    rapier_plugin::{PhysicsWorld, ShapeCastCollisionInfo, ShapeCastCollisionType},
    CollisionData, CollisionEvent, CollisionLayers, CollisionShape, PhysicsSystem,
    RigidBody,
};

use crate::{debug::ENABLE_INSPECTOR, sprites::SpriteSheet, Layer, PrevTransform};

pub const TILE_SIZE: f32 = 10.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        app.add_system_to_stage(CoreStage::Update, move_player);
        app.add_system(active_collisions.after(PhysicsSystem::Events));

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
        // sprites
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: sprites.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..Default::default()
            },
            ..Default::default()
        })
        // collisions
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere {
            radius: TILE_SIZE / 2.,
        })
        .insert(
            CollisionLayers::all_masks::<Layer>()
                .with_groups([Layer::Player, Layer::Unit]),
        )
        .insert(PrevTransform(Transform::default()))
        .insert(ActiveCollisions(HashMap::default()))
        // misc
        .insert(Player { speed: 5.0 })
        .insert(Name::new("Player"))
        .id();
}

#[derive(Component)]
pub struct ActiveCollisions(pub HashMap<Entity, CollisionEvent>);

fn active_collisions(
    mut q: Query<&mut ActiveCollisions>,
    mut collision_evr: EventReader<CollisionEvent>,
) {
    collision_evr
        .iter()
        .for_each(|ev: &CollisionEvent| match ev {
            CollisionEvent::Started(a, b) => {
                if let Ok(mut ac) = q.get_mut(a.rigid_body_entity()) {
                    ac.0.insert(b.rigid_body_entity(), ev.clone());
                }
                if let Ok(mut ac) = q.get_mut(b.rigid_body_entity()) {
                    ac.0.insert(a.rigid_body_entity(), ev.clone());
                }
            }
            CollisionEvent::Stopped(a, b) => {
                if let Ok(mut ac) = q.get_mut(a.rigid_body_entity()) {
                    ac.0.remove(&b.rigid_body_entity());
                }
                if let Ok(mut ac) = q.get_mut(b.rigid_body_entity()) {
                    ac.0.remove(&a.rigid_body_entity());
                }
            }
        })
}

fn move_player(
    mut player_q: Query<(
        Entity,
        &mut Player,
        &mut Transform,
        &GlobalTransform,
        &ActiveCollisions,
    )>,
    physics_world: PhysicsWorld,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if input.get_pressed().len() == 0 {
        return;
    }

    let (player_entity, mut player, mut trans, player_global_trans, _): (
        Entity,
        Mut<Player>,
        Mut<Transform>,
        &GlobalTransform,
        _,
    ) = player_q.single_mut();

    // sum up all keys pressed to get net direction
    let mut dir = Vec3::zeroed();
    for key in input.get_pressed() {
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
    let magnitude = 0.5 * TILE_SIZE * player.speed * time.delta().as_secs_f32();
    let dir = [dir, Vec3::new(dir.x, 0., 0.), Vec3::new(0., dir.y, 0.)]
        .into_iter()
        .find_map(|dir| {
            let cast_res = physics_world.ray_cast_with_filter(
                player_global_trans.translation,
                dir.normalize_or_zero(),
                false,
                // CollisionLayers::all_masks::<Layer>()
                //     .with_groups([Layer::Player, Layer::Unit]),
                CollisionLayers::all::<Layer>(),
                |e| e != player_entity,
            );
            if let Some(hit) = cast_res {
                println!("Hit {:?} - dir: {:?}, ", &hit, dir.xy());
                if hit
                    .collision_point
                    .distance(player_global_trans.translation)
                    > magnitude
                {
                    Some(dir)
                } else {
                    None
                }
            } else {
                println!("No Hit - dir {:?}", dir.xy());
                Some(dir)
            }
        });

    // todo: properly convert between local and global space
    // for &other in player.collided.iter() {
    //     let trans_other = transforms.get(other).unwrap();
    //     let before = trans_other
    //         .translation
    //         .distance(player_global_trans.translation);
    //     let after = trans_other
    //         .translation
    //         .distance(player_global_trans.translation + dir);
    //     if after < before {
    //         println!(
    //             "Input would bring player closer to collided object, blocking. {}",
    //             time.time_since_startup().as_secs_f32()
    //         );
    //         return;
    //     }
    // }
    // multiple the direction by the amplitude to get change in direction
    if let Some(dir) = dir {
        trans.translation += magnitude * dir.normalize();
    }
}
