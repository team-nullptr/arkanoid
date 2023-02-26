use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod actions;
mod assets;
mod ball;
mod block;
mod camera;
mod cursor;
mod lives;
mod menu;
mod paddle;
mod score;
mod ui;
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
        app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
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
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(4.))
            .add_plugin(WorldInspectorPlugin)
            .add_plugin(camera::CameraPlugin)
            .add_plugin(actions::ActionsPlugin)
            .add_plugin(cursor::CursorPlugin)
            .add_plugin(assets::AssetPlugin)
            .add_plugin(paddle::PaddlePlugin)
            .add_plugin(ball::BallPlugin)
            .add_plugin(lives::LivesPlugin)
            .add_plugin(score::PointsPlugin)
            .add_plugin(ui::UiPlugin)
            .add_plugin(menu::MenuPlugin)
            .add_plugin(block::BlockPlugin)
            .add_state(GameState::Loading);
    }
}
