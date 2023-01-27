use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(set_movement_actions)
                .with_system(set_custom_actions),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<f32>,
    pub primary_action: bool,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let player_movement = get_movement(GameControl::Right, &keyboard_input, &mouse_button_input)
        - get_movement(GameControl::Left, &keyboard_input, &mouse_button_input);

    if player_movement != 0. {
        actions.player_movement = Some(player_movement);
    } else {
        actions.player_movement = None;
    }
}

pub fn set_custom_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    actions.primary_action = GameControl::Action.pressed(&keyboard_input, &mouse_button_input);
}
