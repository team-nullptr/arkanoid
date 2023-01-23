use bevy::prelude::*;
use arkanoid::ArkanoidPlugin;

fn main() {
    App::new()
        .add_plugin(ArkanoidPlugin)
        .run();
}
