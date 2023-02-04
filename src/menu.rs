use crate::{assets::FontAssets, util::cleanup, GameState};
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
// const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(play_button_interaction),
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
            parent.spawn(TextBundle::from_section(
                "Arkanoid",
                TextStyle {
                    font: fonts.title_font.clone(),
                    font_size: 64.,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.), Val::Px(65.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(32.)),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
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
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut state: ResMut<State<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            let _ = state.set(GameState::Playing);
        }
    }
}
