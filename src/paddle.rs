use bevy::prelude::*;

use crate::{
    actions::Actions, assets::TextureAssets, cursor::FollowCursor, util::cleanup, GameState,
};

pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_ALTITUDE: f32 = -200.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(paddle_setup))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(paddle_movement))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup::<Paddle>));
    }
}

#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

#[derive(Bundle, Default)]
struct PaddleBundle {
    paddle: Paddle,
    #[bundle]
    sprite: SpriteBundle,
}

fn paddle_setup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn(PaddleBundle {
        sprite: SpriteBundle {
            transform: Transform::from_xyz(0.0, PADDLE_ALTITUDE, 1.0).with_scale(Vec3::splat(2.)),
            texture: texture_assets.paddle.clone(),
            ..default()
        },
		..default()
    });
}

fn paddle_movement(
    mut paddle_query: Query<(&mut Transform, &Handle<Image>), With<Paddle>>,
    cursor_query: Query<&Transform, (With<FollowCursor>, Without<Paddle>, Changed<Transform>)>,
    actions: Res<Actions>,
    time: Res<Time>,
	windows: Res<Windows>,
	images: Res<Assets<Image>>,
) {
    let cursor_position = if let Ok(transform) = cursor_query.get_single() {
        Some(transform.translation)
    } else {
        None
    };

	let window = windows.get_primary().expect("No application window found!");

    for (mut paddle_transform, paddle_texture) in paddle_query.iter_mut() {
        if let Some(cursor_position) = cursor_position {
            paddle_transform.translation.x = cursor_position.x;
        }

        let direction = actions.player_movement.unwrap_or(0.0);

        paddle_transform.translation.x += direction * PADDLE_SPEED * time.delta_seconds();

		let image = images.get(paddle_texture).unwrap();
		let width = image.size().x * paddle_transform.scale.x;
		let bound = (window.width() - width) / 2.;

		paddle_transform.translation.x = paddle_transform.translation.x.clamp(-bound, bound);
    }
}
