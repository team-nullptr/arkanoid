use bevy::prelude::*;

pub struct ArkanoidPlugin;

impl Plugin for ArkanoidPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Arkanoid".to_string(),
				// qHD
				width: 960.0,
				height: 540.0,
				resizable: false,
				..default()
			},
			..default()
		}));
	}
}