use crate::{assets::TextureAssets, ball::BlockHitEvent, util::cleanup, GameState};
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
        // TODO: Improve the silver and gold colors
        match val {
            BlockType::Orange => Color::hex("ff870f").unwrap(),
            BlockType::LightBlue => Color::hex("0fffc3").unwrap(),
            BlockType::Green => Color::hex("219c0b").unwrap(),
            BlockType::Red => Color::hex("a8180d").unwrap(),
            BlockType::Blue => Color::hex("0a13ad").unwrap(),
            BlockType::Pink => Color::hex("c016c9").unwrap(),
            BlockType::Silver { .. } => Color::hex("d9cebd").unwrap(),
            BlockType::Gold => Color::hex("face1e").unwrap(),
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

    let blocks_count = IVec2::new(3, 4);
    let block_margin = Vec2::new(5., 5.);

    let blocks_dims = Vec2::new(
        (blocks_count.x - 1) as f32 * (block_size.x + block_margin.x),
        (blocks_count.y - 1) as f32 * (block_size.y + block_margin.y),
    );

    for i in 0..blocks_count.x {
        for j in 0..blocks_count.y {
            let block_type = match j {
                0 => BlockType::Gold,
                1 => BlockType::Silver { hits_taken: 0 },
                _ => BlockType::Orange,
            };

            commands.spawn(
                BlockBundle::new(block_type, &block_size, textures.block.clone()).with_pos(
                    Vec2::new(
                        (i + 1) as f32 * (block_size.x / 2. + block_margin.x) - blocks_dims.x / 2.,
                        -(j + 1) as f32 * (block_size.y / 2. + block_margin.y) + blocks_dims.y / 2.,
                    ),
                ),
            );
        }
    }
}

fn destroy_blocks(
    mut commands: Commands,
    mut blocks: Query<&mut Block>,
    mut events: EventReader<BlockHitEvent>,
) {
    for event in events.iter() {
        if let Ok(block) = blocks.get(event.0) {
            match block.block_type {
                BlockType::Silver { hits_taken } => {
                    // TODO: Add the level number dependant logic later
                    let hits_taken = hits_taken + 1;

                    if hits_taken >= 2 {
                        commands.entity(event.0).despawn_recursive();
                    } else {
                        blocks.get_mut(event.0).unwrap().block_type =
                            BlockType::Silver { hits_taken };
                    }
                }
                BlockType::Gold => (),
                _ => commands.entity(event.0).despawn_recursive(),
            }
        }
    }
}
