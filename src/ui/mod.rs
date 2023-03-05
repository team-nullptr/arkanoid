use bevy::prelude::*;

pub mod button;
pub mod game_over;
pub mod in_game;
pub mod menu;
pub mod win;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(button::ButtonPlugin)
            .add_plugin(in_game::InGameUiPlugin)
            .add_plugin(menu::MenuPlugin)
            .add_plugin(game_over::GameOverPlugin)
            .add_plugin(win::WinUiPlugin);
    }
}
