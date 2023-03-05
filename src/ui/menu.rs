use crate::{assets::FontAssets, util::cleanup, GameState};
use bevy::prelude::*;

use super::{button::{ArkanoidButtonBundle, ButtonSystem}, set_state_button};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Menu>()
            .register_type::<PlayButton>()
            .register_type::<HelpButton>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(set_state_button::<PlayButton, { GameState::Playing }>)
                    .after(ButtonSystem::UpdateButtonInteraction)
                    .with_system(set_state_button::<HelpButton, { GameState::Help }>)
                    .after(ButtonSystem::UpdateButtonInteraction),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup::<Menu>));
    }
}

#[derive(Component, Reflect)]
pub struct Menu;

#[derive(Component, Reflect)]
pub struct PlayButton;

#[derive(Component, Reflect)]
pub struct HelpButton;

fn spawn_menu(mut commands: Commands, fonts: Res<FontAssets>) {
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
        .insert(Menu)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Arkanoid",
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
                .spawn(ArkanoidButtonBundle::default())
                .insert(PlayButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: fonts.title_font.clone(),
                            font_size: 24.,
                            color: Color::WHITE,
                        },
                    ));
                });

            parent
                .spawn(ArkanoidButtonBundle {
                    button_bundle: ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.), Val::Px(65.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(16.)),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(HelpButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Help",
                        TextStyle {
                            font: fonts.title_font.clone(),
                            font_size: 24.,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}
