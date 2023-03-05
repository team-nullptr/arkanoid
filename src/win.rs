use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{
    block::{Block, BlockType},
    GameState, assets::AudioAssets,
};

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(check_for_win));
    }
}

fn check_for_win(block_query: Query<&Block>, mut state: ResMut<State<GameState>>, audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
    let non_gold_blocks_num = block_query
        .iter()
        .filter(|block| block.block_type != BlockType::Gold)
        .count();

    if non_gold_blocks_num == 0 {
        let _ = state.set(GameState::Win);

        audio.play(audio_assets.win.clone());
    }
}
