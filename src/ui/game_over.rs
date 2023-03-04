use bevy::prelude::*;

use crate::{assets::FontAssets, util::cleanup, GameState};

use super::button::{ArkanoidButtonBundle, ButtonInteraction, ButtonSystem};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameOverUi>()
            .register_type::<GoToMenuButton>()
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(spawn_ui))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(go_to_state_button::<GoToMenuButton, { GameState::Menu }>)
                    .after(ButtonSystem::UpdateButtonInteraction)
                    .with_system(go_to_state_button::<RetryButton, { GameState::Playing }>)
                    .after(ButtonSystem::UpdateButtonInteraction),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver).with_system(cleanup::<GameOverUi>),
            );
    }
}

#[derive(Component, Reflect)]
struct GameOverUi;

#[derive(Component, Reflect)]
struct GoToMenuButton;

#[derive(Component, Reflect)]
struct RetryButton;

fn spawn_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("GameOverUI"))
        .insert(GameOverUi)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Game Over!",
                    TextStyle {
                        font: fonts.title_font.clone(),
                        font_size: 72.,
                        color: Color::rgb(1., 1., 1.),
                    },
                ))
                .insert(Name::new("GameOverUITitle"));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(64.)),
                        justify_content: JustifyContent::SpaceBetween,
                        size: Size::new(Val::Px(256. + 64.), Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ArkanoidButtonBundle::default())
                        .insert(GoToMenuButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Go to menu",
                                TextStyle {
                                    font: fonts.title_font.clone(),
                                    font_size: 24.,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    parent
                        .spawn(ArkanoidButtonBundle::default())
                        .insert(RetryButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Retry",
                                TextStyle {
                                    font: fonts.title_font.clone(),
                                    font_size: 24.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}

fn go_to_state_button<B: Component, const STATE: GameState>(
    mut state: ResMut<State<GameState>>,
    mut query: Query<&ButtonInteraction, (Changed<Interaction>, With<B>)>,
) {
    if let Some(button_interaction) = query.iter_mut().next() {
        if button_interaction.just_released {
            let _ = state.set(STATE);
        }
    }
}
