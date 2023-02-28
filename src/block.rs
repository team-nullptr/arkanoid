use crate::{assets::TextureAssets, ball::BlockHitEvent, score::Score, util::cleanup, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_block))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(destroy_blocks))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup::<Block>));
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

fn spawn_block(mut commands: Commands, textures: Res<TextureAssets>, images: Res<Assets<Image>>) {
    let block_image = images
        .get(&textures.block)
        .expect("Block texture is not loaded");

    let block_size = block_image.size() / 2.;

    let blocks_count = IVec2::new(7, 8);
    let block_gap = Vec2::new(10., 10.);

    let blocks_dims = (Vec2::new(
        blocks_count.x as f32 * block_size.x,
        blocks_count.y as f32 * block_size.y,
    ) + Vec2::new(
        (blocks_count.x - 1) as f32 * block_gap.x,
        (blocks_count.y - 1) as f32 * block_gap.y,
    )) / 2.;

    for i in 0..blocks_count.x {
        for j in 0..blocks_count.y {
            let block_type = match j {
                0 => BlockType::Gold,
                1 => BlockType::Silver { hits_taken: 0 },
                _ => BlockType::Orange,
            };

            commands.spawn(
                BlockBundle::new(block_type, &block_size, textures.block.clone()).with_pos(
                    (Vec2::new(
                        (i as f32 + 0.5) * block_size.x - blocks_dims.x,
                        -(j as f32 + 0.5) * block_size.y + blocks_dims.y,
                    ) + Vec2::new(i as f32 * block_gap.x, -j as f32 * block_gap.y))
                        / 2.,
                ),
            );
        }
    }
}

fn destroy_blocks(
    mut commands: Commands,
    mut blocks: Query<&mut Block>,
    mut paddle_points: Query<&mut Score>,
    mut events: EventReader<BlockHitEvent>,
) {
    let mut paddle_points = paddle_points.single_mut();

    for event in events.iter() {
        if let Ok(mut block) = blocks.get_mut(event.0) {
            let block_type = &mut block.block_type;

            // TODO: Add the level number dependant logic later
            let break_block = match block_type {
                BlockType::Silver { hits_taken } => {
                    *hits_taken += 1;

                    *hits_taken >= 2
                }
                BlockType::Gold => false,
                _ => true,
            };

            if break_block {
                commands.entity(event.0).despawn_recursive();

                **paddle_points += block_type.score(1);
            }
        }
    }
}
