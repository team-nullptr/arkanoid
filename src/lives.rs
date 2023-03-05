use bevy::prelude::*;

use crate::{assets::TextureAssets, ui::in_game::LivesUi, GameState};

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lives>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(display_lives));
    }
}

#[derive(Component, Reflect, Copy, Clone, PartialEq, Debug)]
pub struct Lives {
    lives: u32,
}

impl Default for Lives {
    fn default() -> Self {
        Self { lives: 3 }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Deref)]
pub struct LivesReachedZero {
    lives_reached_zero: bool,
}

impl Lives {
    #[must_use]
    pub fn lose(&mut self, amount: u32) -> LivesReachedZero {
        self.lives -= amount;

        LivesReachedZero {
            lives_reached_zero: self.lives == 0,
        }
    }
}

impl LivesReachedZero {
    pub fn lives_reached_zero(&self) -> bool {
        self.lives_reached_zero
    }
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
struct LivesUiElement;

fn display_lives(
    mut commands: Commands,
    lives_query: Query<&Lives, Changed<Lives>>,
    ui_query: Query<Entity, With<LivesUi>>,
    ui_children_query: Query<Entity, With<LivesUiElement>>,
    texture_assets: Res<TextureAssets>,
) {
    if let Ok(lives) = lives_query.get_single() {
        for child in ui_children_query.iter() {
            commands.entity(child).despawn_recursive();
        }

        if let Ok(ui) = ui_query.get_single() {
            commands.entity(ui).with_children(|parent| {
                for _ in 0..lives.lives {
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                margin: UiRect::left(Val::Px(8.)),
                                ..default()
                            },
                            image: texture_assets.ball_small.clone().into(),
                            ..default()
                        })
                        .insert(LivesUiElement);
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lose_lives() {
        let mut lives = Lives::default();

        assert_eq!(lives.lives, 3);

        let _ = lives.lose(1);

        assert_eq!(lives.lives, 2);

        let _ = lives.lose(2);

        assert_eq!(lives.lives, 0);
    }

    #[test]
    fn lives_reached_zero() {
        let mut lives = Lives::default();

        assert_eq!(lives.lives, 3);

        assert!(!lives.lose(1).lives_reached_zero());

        assert!(lives.lose(2).lives_reached_zero());
    }
}
