use crate::{assets::FontAssets, util::cleanup, GameState};
use bevy::prelude::*;

use super::button::{ArkanoidButtonBundle, ButtonInteraction, ButtonSystem};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(play_button_interaction)
                    .after(ButtonSystem::UpdateButtonInteraction),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup::<Menu>));
    }
}

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct PlayButton;

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
        });
}

fn play_button_interaction(
    mut interaction_query: Query<&ButtonInteraction, (Changed<Interaction>, With<PlayButton>)>,
    mut state: ResMut<State<GameState>>,
) {
    for button_interaction in &mut interaction_query {
        if button_interaction.just_released {
            let _ = state.set(GameState::Playing);
        }
    }
}
