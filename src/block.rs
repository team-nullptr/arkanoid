use crate::{
    assets::{AudioAssets, LevelAssets, TextureAssets},
    ball::BlockHitEvent,
    level::{CurrentLevel, LevelAsset},
    score::Score,
    util::cleanup,
    GameState,
};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(load_current_level))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(destroy_blocks))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup::<Block>))
            .add_system_set(SystemSet::on_exit(GameState::Win).with_system(cleanup::<Block>));
    }
}

#[derive(Reflect, Copy, Clone, PartialEq, Debug)]
pub enum BlockType {
    Orange,
    LightBlue,
    Green,
    Red,
    Blue,
    Pink,
    Silver { hits_taken: u32 },
    Gold,
}

impl From<BlockType> for Color {
    fn from(val: BlockType) -> Self {
        match val {
            BlockType::Orange => Color::hex("ff870f").unwrap(),
            BlockType::LightBlue => Color::hex("0fffc3").unwrap(),
            BlockType::Green => Color::hex("219c0b").unwrap(),
            BlockType::Red => Color::hex("a8180d").unwrap(),
            BlockType::Blue => Color::hex("0a13ad").unwrap(),
            BlockType::Pink => Color::hex("c016c9").unwrap(),
            BlockType::Silver { .. } => Color::hex("c5ced4").unwrap(),
            BlockType::Gold => Color::hex("d4af37").unwrap(),
        }
    }
}

impl BlockType {
    fn score(&self, level_num: u32) -> u32 {
        match *self {
            BlockType::Orange => 60,
            BlockType::LightBlue => 70,
            BlockType::Green => 80,
            BlockType::Red => 90,
            BlockType::Blue => 100,
            BlockType::Pink => 110,
            BlockType::Silver { .. } => 50 * level_num,
            BlockType::Gold => 120,
        }
    }
}

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug)]
pub struct Block {
    pub block_type: BlockType,
}

#[derive(Bundle)]
pub struct BlockBundle {
    block: Block,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
}

impl BlockBundle {
    fn new(block_type: BlockType, block_size: &Vec2, texture: Handle<Image>) -> Self {
        Self {
            block: Block { block_type },
            collider: Collider::cuboid(block_size.x, block_size.y),
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_scale(Vec3::splat(0.25)),
                sprite: Sprite {
                    color: block_type.into(),
                    ..default()
                },
                ..default()
            },
        }
    }

    fn with_pos(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = pos.extend(0.0);
        self
    }
}

fn load_current_level(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    images: Res<Assets<Image>>,
    level_assets: Res<LevelAssets>,
    levels: Res<Assets<LevelAsset>>,
    current_level: Res<CurrentLevel>,
) {
    let level = levels.get(&level_assets.levels[current_level.0]).unwrap();

    let level_height = level.tiles.len();

    let block_image = images
        .get(&textures.block)
        .expect("Block texture is not loaded");

    let block_size = block_image.size() / 2.;

    let block_gap = Vec2::new(10., 10.);

    for i in 0..level_height {
        let level_physical_height =
            level_height as f32 * block_size.y + (level_height - 1) as f32 * block_gap.y;

        let level_width = level.tiles[i].len();

        let level_physical_width =
            level_width as f32 * block_size.x + (level_width - 1) as f32 * block_gap.x;

        for j in 0..level_width {
            let block_type = match level.tiles[i][j].as_str() {
                "silver" => BlockType::Silver { hits_taken: 0 },
                "gold" => BlockType::Gold,
                "orange" => BlockType::Orange,
                "lightblue" => BlockType::LightBlue,
                "green" => BlockType::Green,
                "red" => BlockType::Red,
                "blue" => BlockType::Blue,
                "pink" => BlockType::Pink,
                "blank" => continue,
                _ => panic!("Invalid block type: {}", level.tiles[i][j]),
            };

            commands.spawn(
                BlockBundle::new(block_type, &block_size, textures.block.clone()).with_pos(
                    Vec2::new(
                        -level_physical_width / 2.
                            + block_size.x / 2.
                            + j as f32 * block_size.x
                            + j as f32 * block_gap.x,
                        level_physical_height / 2.
                            - block_size.y / 2.
                            - i as f32 * block_size.y
                            - i as f32 * block_gap.y,
                    ) / 2.,
                ),
            );
        }
    }
}

fn destroy_blocks(
    mut commands: Commands,
    mut blocks: Query<&mut Block>,
    mut paddle_points: Query<&mut Score>,
    current_level: Res<CurrentLevel>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut events: EventReader<BlockHitEvent>,
) {
    let mut paddle_points = paddle_points.single_mut();

    for event in events.iter() {
        if let Ok(mut block) = blocks.get_mut(event.0) {
            let block_type = &mut block.block_type;

            let break_block = match block_type {
                BlockType::Silver { hits_taken } => {
                    *hits_taken += 1;

                    *hits_taken >= current_level.0 as u32 / 8 + 2
                }
                BlockType::Gold => false,
                _ => true,
            };

            if break_block {
                commands.entity(event.0).despawn_recursive();

                **paddle_points += block_type.score(current_level.0 as u32);

                audio.play(audio_assets.block_break.clone());
            } else {
                audio.play(audio_assets.block_bounce.clone());
            }
        }
    }
}
