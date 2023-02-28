//! Defines the in-game UI layout.

use bevy::prelude::*;

use crate::{assets::FontAssets, util::cleanup, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InGameUi>()
            .register_type::<LivesUi>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_ui))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup::<InGameUi>),
            );
    }
}

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug, Default)]
pub struct InGameUi;

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug, Default)]
pub struct LivesUi;

/// A marker component for the score UI element.
#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug, Default)]
pub struct ScoreUi;

fn setup_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .insert(InGameUi)
        .insert(Name::new("UI"))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Name::new("LeftSection"))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "0",
                            TextStyle {
                                font: fonts.title_font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(Name::new("Score"))
                        .insert(ScoreUi);
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Name::new("MiddleSection"));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Name::new("RightSection"))
                .with_children(|right_section| {
                    right_section
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::RowReverse,
                                size: Size::new(Val::Percent(100.0), Val::Px(32.0)),
                                ..Default::default()
                            },
                            ..default()
                        })
                        .insert(LivesUi)
                        .insert(Name::new("Lives"));
                });
        });
}
