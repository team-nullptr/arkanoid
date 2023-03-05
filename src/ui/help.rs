use bevy::prelude::*;

use crate::{
    assets::{FontAssets, TextureAssets},
    util::cleanup,
    GameState,
};

use super::{button::ArkanoidButtonBundle, set_state_button};

pub struct HelpPlugin;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HelpUi>()
            .register_type::<GoToMenuButton>()
            .add_system_set(SystemSet::on_enter(GameState::Help).with_system(setup_help_ui))
            .add_system_set(
                SystemSet::on_update(GameState::Help)
                    .with_system(set_state_button::<GoToMenuButton, { GameState::Menu }>),
            )
            .add_system_set(SystemSet::on_exit(GameState::Help).with_system(cleanup::<HelpUi>));
    }
}

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug)]
struct HelpUi;

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug)]
struct GoToMenuButton;

fn setup_help_ui(mut commands: Commands, images: Res<TextureAssets>, fonts: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(HelpUi)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Help",
                    TextStyle {
                        font: fonts.title_font.clone(),
                        font_size: 64.,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(32.)),
                    ..Default::default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Break all the blocks by bouncing the ball off the paddle.",
                        TextStyle {
                            font: fonts.title_font.clone(),
                            font_size: 24.,
                            color: Color::WHITE,
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(16.)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle::default()).with_children(|parent| {
                                parent.spawn(NodeBundle::default()).with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(48.), Val::Px(48.)),
                                            ..default()
                                        },
                                        image: images.arrow_left_key_icon.clone().into(),
                                        ..default()
                                    });

                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(48.), Val::Px(48.)),
                                            ..default()
                                        },
                                        image: images.arrow_right_key_icon.clone().into(),
                                        ..default()
                                    });
                                });

                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            margin: UiRect::left(Val::Px(12.)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(48.), Val::Px(48.)),
                                                ..default()
                                            },
                                            image: images.a_key_icon.clone().into(),
                                            ..default()
                                        });

                                        parent.spawn(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(48.), Val::Px(48.)),
                                                ..default()
                                            },
                                            image: images.d_key_icon.clone().into(),
                                            ..default()
                                        });
                                    });

                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            margin: UiRect::left(Val::Px(12.)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(48.), Val::Px(48.)),
                                                ..default()
                                            },
                                            image: images.mouse_icon.clone().into(),
                                            ..default()
                                        });
                                    });
                            });

                            parent.spawn(
                                TextBundle::from_section(
                                    "Move the paddle left and right",
                                    TextStyle {
                                        font: fonts.title_font.clone(),
                                        font_size: 24.,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::left(Val::Px(32.)),
                                    ..default()
                                }),
                            );
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(8.), Val::Px(32.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle::default()).with_children(|parent| {
                        parent.spawn(NodeBundle::default()).with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(48.), Val::Px(48.)),
                                    ..default()
                                },
                                image: images.space_icon.clone().into(),
                                ..default()
                            });
                        });

                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    margin: UiRect::left(Val::Px(16.)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(48.), Val::Px(48.)),
                                        ..default()
                                    },
                                    image: images.left_mouse_button_icon.clone().into(),
                                    ..default()
                                });
                            });
                    });

                    parent.spawn(
                        TextBundle::from_section(
                            "Shoot the ball (when glued to the paddle)",
                            TextStyle {
                                font: fonts.title_font.clone(),
                                font_size: 24.,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::left(Val::Px(32.)),
                            ..default()
                        }),
                    );
                });

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
        });
}
