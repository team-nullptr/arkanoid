use bevy::prelude::*;

use crate::{ui::LivesUi, assets::TextureAssets, GameState};

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Lives>()
			.add_system_set(SystemSet::on_update(GameState::Playing)
			.with_system(display_lives));
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
pub struct LivesReachedZero(bool);

impl Lives {
	#[must_use]
	fn lose(&mut self, amount: u32) -> LivesReachedZero {
		self.lives -= amount;

		LivesReachedZero(self.lives <= 0)
	}

	fn gain(&mut self, amount: u32) {
		self.lives += amount;
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

		commands.entity(ui_query.single()).with_children(|parent| {
			for _ in 0..lives.lives {
				parent.spawn(ImageBundle {
					style: Style {
						margin: UiRect::left(Val::Px(8.)),
						..default()
					},
					image: texture_assets.ball_small.clone().into(),
					..default()
				}).insert(LivesUiElement);
			}
		});
	}
}