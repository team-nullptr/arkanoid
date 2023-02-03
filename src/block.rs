use crate::{assets::TextureAssets, util::cleanup, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_block))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup::<Block>));
    }
}

#[derive(Component)]
pub struct Block;

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
        block: Block,
        sprite: SpriteBundle {
            texture: textures.block.clone(),
            ..default()
        },
        collider: Collider::cuboid(block_size.x / 2.0, block_size.y / 2.0),
    });
}
