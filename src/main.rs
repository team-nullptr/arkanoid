use bevy::prelude::*;

fn main() {
    App::new()
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
        .run();
}
