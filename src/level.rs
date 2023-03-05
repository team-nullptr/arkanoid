use bevy::{prelude::*, reflect::TypeUuid, asset::{LoadContext, AssetLoader, LoadedAsset}, utils::BoxedFuture};

use serde::Deserialize;

use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset_loader::<LevelLoader>()
			.add_asset::<LevelAsset>()
			.init_resource::<CurrentLevel>()
			.add_system_set(
				SystemSet::on_exit(GameState::Menu)
					.with_system(reset_current_level),
			);
	}
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CurrentLevel(pub usize);

fn reset_current_level(mut current_level: ResMut<CurrentLevel>) {
	current_level.0 = 0;
}

#[derive(Debug, Deserialize, Reflect, TypeUuid)]
#[uuid = "5c8be95c-5d54-46d2-a903-ac7bc7d5b4c2"]
pub struct LevelAsset {
	pub tiles: Vec<Vec<String>>,
}

#[derive(Default)]
pub struct LevelLoader;

impl AssetLoader for LevelLoader {
	fn load<'a>(
		&self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move {
			let level = serde_json::from_slice::<LevelAsset>(bytes)?;
			load_context.set_default_asset(LoadedAsset::new(level));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["lvl"]
	}
}