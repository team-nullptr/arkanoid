use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{GameState, level::LevelAsset};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<LevelAssets>()
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
    #[asset(path = "img/input-icons/Arrow_Left_Key_Light.png")]
    pub arrow_left_key_icon: Handle<Image>,
    #[asset(path = "img/input-icons/Arrow_Right_Key_Light.png")]
    pub arrow_right_key_icon: Handle<Image>,
    #[asset(path = "img/input-icons/A_Key_Light.png")]
    pub a_key_icon: Handle<Image>,
    #[asset(path = "img/input-icons/D_Key_Light.png")]
    pub d_key_icon: Handle<Image>,
    #[asset(path = "img/input-icons/Mouse_Simple_Key_Light.png")]
    pub mouse_icon: Handle<Image>,
    #[asset(path = "img/input-icons/Space_Key_Light.png")]
    pub space_icon: Handle<Image>,
    #[asset(path = "img/input-icons/Mouse_Left_Key_Light.png")]
    pub left_mouse_button_icon: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "levels/level1.lvl")]
    pub level1: Handle<LevelAsset>,
}
