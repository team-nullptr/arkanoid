use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Unbounded-Medium.ttf")]
    pub title_font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "img/paddle.png")]
    pub paddle: Handle<Image>,
    #[asset(path = "img/ball.png")]
    pub ball: Handle<Image>,
    #[asset(path = "img/ball_small.png")]
    pub ball_small: Handle<Image>,
    #[asset(path = "img/block.png")]
    pub block: Handle<Image>,
}
