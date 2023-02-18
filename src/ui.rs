//! Defines the in-game UI layout.

use bevy::prelude::*;

use crate::{util::cleanup, GameState};

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

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .insert(InGameUi)
        .insert(Name::new("UI"))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: Val::Percent(100.0),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Name::new("LeftSection"));

            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: Val::Percent(100.0),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Name::new("MiddleSection"));

            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: Val::Percent(100.0),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Name::new("RightSection"))
            .with_children(|right_section| {
                right_section.spawn(NodeBundle {
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
