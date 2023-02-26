use bevy::prelude::{Input, KeyCode, MouseButton, Res};

pub enum GameControl {
    Left,
    Right,
    Action,
}

impl GameControl {
    pub fn pressed(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        mouse_button_input: &Res<Input<MouseButton>>,
    ) -> bool {
        match self {
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
            GameControl::Action => {
                keyboard_input.just_pressed(KeyCode::Space)
                    || mouse_button_input.just_pressed(MouseButton::Left)
            }
        }
    }
}

pub fn get_movement(
    control: GameControl,
    input: &Res<Input<KeyCode>>,
    mouse_button_input: &Res<Input<MouseButton>>,
) -> f32 {
    if control.pressed(input, mouse_button_input) {
        1.0
    } else {
        0.0
    }
}
