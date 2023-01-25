use bevy::prelude::*;

mod actions;
mod assets;
mod camera;
mod cursor;
mod paddle;
mod util;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing,
}

pub struct ArkanoidPlugin;

impl Plugin for ArkanoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Arkanoid".to_string(),
                    // qHD
                    width: 960.0,
                    height: 540.0,
                    resizable: false,
                    ..default()
                },
                ..default()
            }))
            .add_plugin(camera::CameraPlugin)
            .add_plugin(actions::ActionsPlugin)
            .add_plugin(cursor::CursorPlugin)
            .add_plugin(assets::AssetPlugin)
            .add_plugin(paddle::PaddlePlugin)
            .add_state(GameState::Loading);
    }
}
