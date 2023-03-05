use bevy::prelude::*;

use crate::{
    assets::{FontAssets, LevelAssets},
    level::CurrentLevel,
    util::cleanup,
    GameState,
};

use super::{
    button::{ArkanoidButtonBundle, ButtonInteraction, ButtonSystem},
    set_state_button,
};

pub struct WinUiPlugin;

impl Plugin for WinUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WinUi>()
            .register_type::<GoToMenuButton>()
            .register_type::<NextLevelButton>()
            .add_system_set(SystemSet::on_enter(GameState::Win).with_system(spawn_ui))
            .add_system_set(
                SystemSet::on_update(GameState::Win)
                    .with_system(set_state_button::<GoToMenuButton, { GameState::Menu }>)
                    .after(ButtonSystem::UpdateButtonInteraction)
                    .with_system(go_to_next_level)
                    .after(ButtonSystem::UpdateButtonInteraction),
            )
            .add_system_set(SystemSet::on_exit(GameState::Win).with_system(cleanup::<WinUi>));
    }
}

#[derive(Component, Reflect)]
struct WinUi;

#[derive(Component, Reflect)]
struct GoToMenuButton;

#[derive(Component, Reflect)]
struct NextLevelButton;

fn spawn_ui(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    level_assets: Res<LevelAssets>,
    current_level: Res<CurrentLevel>,
) {
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
        .insert(WinUi)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "You win!",
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

                    if level_assets.levels.len() > current_level.0 + 1 {
                        parent
                            .spawn(ArkanoidButtonBundle {
                                button_bundle: ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.), Val::Px(65.)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::left(Val::Px(16.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(NextLevelButton)
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Next Level",
                                    TextStyle {
                                        font: fonts.title_font.clone(),
                                        font_size: 24.,
                                        color: Color::WHITE,
                                    },
                                ));
                            });
                    }
                });
        });
}

fn go_to_next_level(
    next_level_query: Query<&ButtonInteraction, With<NextLevelButton>>,
    mut current_level: ResMut<CurrentLevel>,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok(button_interaction) = next_level_query.get_single() {
        if button_interaction.just_released {
            current_level.0 += 1;
            let _ = state.set(GameState::Playing);
        }
    }
}
