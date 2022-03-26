use std::default::Default;

use bevy::prelude::*;

pub struct SpriteSheet(pub Handle<TextureAtlas>);

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            load_sprite_sheet,
        );
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_fonts);
        app.add_startup_system_to_stage(
            StartupStage::PostStartup,
            draw_sprites_with_ids,
        );
    }
}

pub fn write(
    commands: &mut Commands,
    font: &Res<MediumFont>,
    text: &str,
    position: Vec2,
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: font.0.clone(),
                font_size: 12.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            },
        ),
        transform: Transform {
            translation: Vec3::from((position, 950.)),
            scale: Vec3::new(0.003, 0.003, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub struct MediumFont(pub Handle<Font>);

fn load_fonts(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load::<Font, _>("fonts/FiraMono-Medium.ttf");
    commands.insert_resource(MediumFont(font));
}

fn load_sprite_sheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("1bitpack_kenny/Tilesheet/colored.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(16.),
        49,
        22,
        Vec2::splat(1.),
    );
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(SpriteSheet(atlas_handle));
}

fn col_row(col: usize, row: usize) -> usize {
    col + row * 49
}

fn draw_sprites_with_ids(
    mut commands: Commands,
    sheet: Res<SpriteSheet>,
    font: Res<MediumFont>,
) {
    write(&mut commands, &font, "Testing!", Vec2::new(0., 0.));
    for row in 0..22 {
        for col in 0..49 {
            let mut sprite = TextureAtlasSprite::new(col_row(col, row));
            sprite.custom_size = Some(Vec2::splat(0.05));

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite,
                    texture_atlas: sheet.0.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            col as f32 * 0.07 - 1.0,
                            row as f32 * 0.07 - 1.0,
                            100.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new(format!("Sprite {}.{}", col, row)))
                .id();

            write(
                &mut commands,
                &font,
                &format!("{}", col_row(col, row)),
                Vec2::new(col as f32 - 0.1, row as f32 - 0.2) * 0.07
                    - Vec2::splat(1.0),
            );
        }
    }
}
