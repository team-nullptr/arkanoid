use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .init_resource::<Actions>()
            .add_system(set_movement_actions)
            .add_system(call_input_events);
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<f32>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum InputEvent {
    PrimaryAction,
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

pub fn call_input_events(
    mut input_events: EventWriter<InputEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if GameControl::Action.pressed(&keyboard_input, &mouse_button_input) {
        input_events.send(InputEvent::PrimaryAction);
    }
}
