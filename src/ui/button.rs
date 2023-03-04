use bevy::prelude::*;

pub const DEFAULT_NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const DEFAULT_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const DEFAULT_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ButtonInteraction>()
            .register_type::<StyledButton>()
            .add_system(styled_button_system.label(ButtonSystem::StyledButton))
            .add_system(update_button_interaction.label(ButtonSystem::UpdateButtonInteraction));
    }
}

#[derive(SystemLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonSystem {
    StyledButton,
    UpdateButtonInteraction,
}

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug)]
pub struct ButtonInteraction {
    pub just_pressed: bool,
    pub just_released: bool,
    pub previous_interaction_state: Interaction,
}

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug)]
pub struct StyledButton {
    pub normal_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
}

impl Default for StyledButton {
    fn default() -> Self {
        Self {
            normal_color: DEFAULT_NORMAL_BUTTON_COLOR,
            hovered_color: DEFAULT_HOVERED_BUTTON_COLOR,
            pressed_color: DEFAULT_PRESSED_BUTTON_COLOR,
        }
    }
}

#[derive(Bundle)]
pub struct ArkanoidButtonBundle {
    #[bundle]
    pub button_bundle: ButtonBundle,
    pub styled_button: StyledButton,
    pub button_interaction: ButtonInteraction,
}

impl Default for ArkanoidButtonBundle {
    fn default() -> Self {
        Self {
            button_bundle: ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.), Val::Px(65.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: DEFAULT_NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            },
            styled_button: StyledButton::default(),
            button_interaction: ButtonInteraction {
                just_pressed: false,
                just_released: false,
                previous_interaction_state: Interaction::None,
            },
        }
    }
}

pub fn styled_button_system(
    mut button_query: Query<
        (&StyledButton, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (button, interaction, mut background_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *background_color = button.pressed_color.into();
            }
            Interaction::Hovered => {
                *background_color = button.hovered_color.into();
            }
            Interaction::None => {
                *background_color = button.normal_color.into();
            }
        }
    }
}

fn update_button_interaction(
    mut interaction_query: Query<(&mut ButtonInteraction, &Interaction), Changed<Interaction>>,
) {
    for (mut button_interaction, interaction) in interaction_query.iter_mut() {
        button_interaction.just_released = *interaction != Interaction::Clicked
            && button_interaction.previous_interaction_state == Interaction::Clicked;

        button_interaction.just_released = *interaction != Interaction::Clicked
            && button_interaction.previous_interaction_state == Interaction::Clicked;

        button_interaction.previous_interaction_state = *interaction;
    }
}
