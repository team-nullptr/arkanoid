use bevy::prelude::*;

use crate::GameState;

use self::button::ButtonInteraction;

pub mod button;
pub mod game_over;
pub mod help;
pub mod in_game;
pub mod menu;
pub mod win;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(button::ButtonPlugin)
            .add_plugin(in_game::InGameUiPlugin)
            .add_plugin(menu::MenuPlugin)
            .add_plugin(help::HelpPlugin)
            .add_plugin(game_over::GameOverPlugin)
            .add_plugin(win::WinUiPlugin);
    }
}

pub fn set_state_button<B: Component, const STATE: GameState>(
    mut state: ResMut<State<GameState>>,
    mut query: Query<&ButtonInteraction, (Changed<Interaction>, With<B>)>,
) {
    if let Some(button_interaction) = query.iter_mut().next() {
        if button_interaction.just_released {
            let _ = state.set(STATE);
        }
    }
}
