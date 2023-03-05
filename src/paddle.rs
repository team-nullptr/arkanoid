use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::*;

use crate::{
    actions::Actions,
    assets::{AudioAssets, TextureAssets},
    ball::{Ball, BallResetEvent},
    cursor::FollowCursor,
    lives::Lives,
    score::Score,
    util::cleanup,
    GameState,
};

pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_ALTITUDE: f32 = -200.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(paddle_setup.label(PaddleSystem::Setup)),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(paddle_movement.label(PaddleSystem::Movement))
                .with_system(lose_lives.label(PaddleSystem::LoseLives)),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(cleanup::<Paddle>.label(PaddleSystem::Cleanup)),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Win)
                .with_system(cleanup::<Paddle>.label(PaddleSystem::Cleanup)),
        );
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum PaddleSystem {
    Setup,
    Movement,
    LoseLives,
    Cleanup,
}

#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

#[derive(Bundle, Default)]
pub struct PaddleBundle {
    paddle: Paddle,
    name: Name,
    lives: Lives,
    points: Score,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
}

fn paddle_setup(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    images: Res<Assets<Image>>,
) {
    let image = images
        .get(&texture_assets.paddle)
        .expect("Paddle texture not loaded yet!");
    let paddle_size = image.size();

    commands.spawn(PaddleBundle {
        name: Name::new("Paddle"),
        sprite: SpriteBundle {
            transform: Transform::from_xyz(0.0, PADDLE_ALTITUDE, 1.0).with_scale(Vec3::splat(0.25)),
            texture: texture_assets.paddle.clone(),
            ..default()
        },
        collider: Collider::cuboid(paddle_size.x / 2., paddle_size.y / 2.),
        ..default()
    });
}

fn paddle_movement(
    mut paddle_query: Query<(&mut Transform, &Collider), With<Paddle>>,
    cursor_query: Query<&Transform, (With<FollowCursor>, Without<Paddle>, Changed<Transform>)>,
    actions: Res<Actions>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    let cursor_position = if let Ok(transform) = cursor_query.get_single() {
        Some(transform.translation)
    } else {
        None
    };

    let window_width = windows.get_primary().map(|window| window.width());

    for (mut paddle_transform, paddle_collider) in paddle_query.iter_mut() {
        if let Some(cursor_position) = cursor_position {
            paddle_transform.translation.x = cursor_position.x;
        }

        let direction = actions.player_movement.unwrap_or(0.0);

        paddle_transform.translation.x += direction * PADDLE_SPEED * time.delta_seconds();

        if let Some(window_width) = window_width {
            let bound = window_width / 2.
                - paddle_collider
                    .as_cuboid()
                    .expect("The paddle collider is not a cuboid!")
                    .half_extents()
                    .x;

            if bound < 0. {
                panic!("Paddle is too big for the window!");
            }

            paddle_transform.translation.x = paddle_transform.translation.x.clamp(-bound, bound);
        }
    }
}

fn lose_lives(
    mut state: ResMut<State<GameState>>,
    mut lives_query: Query<&mut Lives>,
    ball_query: Query<&Transform, With<Ball>>,
    windows: Res<Windows>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut ball_reset_event_writer: EventWriter<BallResetEvent>,
) {
    let window = windows.get_primary().expect("Primary window not found");
    let transform = ball_query.single();

    if transform.translation.y < -window.height() / 2. {
        for mut lives in lives_query.iter_mut() {
            if lives.lose(1).lives_reached_zero() {
                let _ = state.set(GameState::GameOver);

                audio.play(audio_assets.lose.clone());
            } else {
                audio.play(audio_assets.lose_live.clone());
            }

            ball_reset_event_writer.send(BallResetEvent);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn right_movement_test() {
        let mut world = World::new();

        let paddle_x = 0.0;

        world.spawn(PaddleBundle {
            name: Name::new("Paddle"),
            sprite: SpriteBundle {
                transform: Transform::from_xyz(paddle_x, 0.0, 1.0),
                ..default()
            },
            ..default()
        });

        let mut update_stage = SystemStage::parallel();

        update_stage.add_system(paddle_movement);

        let actions = Actions {
            player_movement: Some(1.0),
        };

        world.insert_resource(actions);

        let mut time = Time::default();
        time.update();
        world.insert_resource(time);

        world.init_resource::<Windows>();

        let mut time = world.resource_mut::<Time>();
        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + Duration::from_secs(1));

        update_stage.run(&mut world);

        let paddle_transform = world
            .query_filtered::<&Transform, With<Paddle>>()
            .single(&world);

        assert_eq!(paddle_transform.translation.x, paddle_x + PADDLE_SPEED);
    }

    #[test]
    fn left_movement_test() {
        let mut world = World::new();

        let paddle_x = 0.0;

        world.spawn(PaddleBundle {
            name: Name::new("Paddle"),
            sprite: SpriteBundle {
                transform: Transform::from_xyz(paddle_x, 0.0, 1.0),
                ..default()
            },
            ..default()
        });

        let mut update_stage = SystemStage::parallel();

        update_stage.add_system(paddle_movement);

        let actions = Actions {
            player_movement: Some(-1.0),
        };

        world.insert_resource(actions);

        let mut time = Time::default();
        time.update();
        world.insert_resource(time);

        world.init_resource::<Windows>();

        let mut time = world.resource_mut::<Time>();
        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + Duration::from_secs(1));

        update_stage.run(&mut world);

        let paddle_transform = world
            .query_filtered::<&Transform, With<Paddle>>()
            .single(&world);

        assert_eq!(paddle_transform.translation.x, paddle_x - PADDLE_SPEED);
    }
}
