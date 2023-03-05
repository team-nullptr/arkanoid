use bevy::{prelude::*, reflect::TypeUuid, asset::{LoadContext, AssetLoader, LoadedAsset}, utils::BoxedFuture};

use serde::Deserialize;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset_loader::<LevelLoader>()
			.add_asset::<LevelAsset>();
	}
}

#[derive(Debug, Deserialize, Reflect, TypeUuid)]
#[uuid = "5c8be95c-5d54-46d2-a903-ac7bc7d5b4c2"]
pub struct LevelAsset {
	pub width: u32,
	pub height: u32,
	pub tiles: Vec<String>,
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