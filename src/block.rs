use crate::{assets::TextureAssets, GameState, util::cleanup, ball::BlockHitEvent};
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
    Silver {
        hits_taken: u32,
    },
    Gold,
}

impl Into<Color> for BlockType {
    fn into(self) -> Color {
        match self {
            BlockType::Orange => Color::rgb(1.0, 0.5, 0.0),
            BlockType::LightBlue => Color::rgb(0.0, 0.5, 1.0),
            BlockType::Green => Color::rgb(0.0, 1.0, 0.0),
            BlockType::Red => Color::rgb(1.0, 0.0, 0.0),
            BlockType::Blue => Color::rgb(0.0, 0.0, 1.0),
            BlockType::Pink => Color::rgb(1.0, 0.0, 1.0),
            BlockType::Silver { .. } => Color::rgb(0.75, 0.75, 0.75),
            BlockType::Gold => Color::rgb(1.0, 0.75, 0.0),
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

fn spawn_block(mut commands: Commands, textures: Res<TextureAssets>, images: Res<Assets<Image>>) {
    let block_image = images
        .get(&textures.block)
        .expect("Block texture is not loaded");

    let block_size = block_image.size();

    commands.spawn(BlockBundle {
        block: Block {
            block_type: BlockType::Orange,
        },
        sprite: SpriteBundle {
            texture: textures.block.clone(),
            transform: Transform::from_scale(Vec3::splat(0.25)),
            ..default()
        },
        collider: Collider::cuboid(block_size.x / 2.0, block_size.y / 2.0),
    });
}

fn destroy_blocks(mut commands: Commands, mut blocks: Query<&mut Block>, mut events: EventReader<BlockHitEvent>) {
    for event in events.iter() {
        if let Ok(block) = blocks.get(event.0) {
            match block.block_type {
                BlockType::Silver { hits_taken } => {
                    // TODO: Add the level number dependant logic later
                    let hits_taken = hits_taken + 1;

                    if hits_taken >= 2 {
                        commands.entity(event.0).despawn_recursive();
                    } else {
                        blocks.get_mut(event.0).unwrap().block_type = BlockType::Silver { hits_taken };
                    }
                },
                BlockType::Gold => (),
                _ => commands.entity(event.0).despawn_recursive(),
            }
        }
    }
}
