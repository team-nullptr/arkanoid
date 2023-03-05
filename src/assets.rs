use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

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
pub struct AudioAssets {
    #[asset(path = "audio/bounce.wav")]
    pub bounce: Handle<AudioSource>,
    #[asset(path = "audio/block_break.wav")]
    pub block_break: Handle<AudioSource>,
    #[asset(path = "audio/block_bounce.wav")]
    pub block_bounce: Handle<AudioSource>,
    #[asset(path = "audio/win.wav")]
    pub win: Handle<AudioSource>,
    #[asset(path = "audio/lose.wav")]
    pub lose: Handle<AudioSource>,
    #[asset(path = "audio/lose_live.wav")]
    pub lose_live: Handle<AudioSource>,
}

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
