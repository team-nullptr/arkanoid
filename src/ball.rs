use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    actions::Actions,
    assets::TextureAssets,
    paddle::{Paddle, PaddleSystem},
    util::cleanup,
    GameState,
};

pub const DEFAULT_BALL_SPEED: f32 = 300.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(ball_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(ball_movement)
                    .after(PaddleSystem::Movement)
                    .with_system(ball_control),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup::<Ball>));
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BallState {
    Glued {
        /// The percentage of the paddle's width that the ball is glued to.
        /// 0.0 is the left edge, 1.0 is the right edge.
        percentage: f32,
    },
    Free,
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Ball {
    pub direction: Vec2,
    pub speed: f32,
    pub state: BallState,
}

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
}

fn ball_setup(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    images: Res<Assets<Image>>,
) {
    let image = images
        .get(&texture_assets.ball)
        .expect("Ball texture not loaded yet!");

    let ball_size = image.size();

    commands.spawn(BallBundle {
        ball: Ball {
            direction: Vec2::new(0., 1.).normalize(),
            speed: DEFAULT_BALL_SPEED,
            state: BallState::Glued { percentage: 0.5 },
        },
        collider: Collider::ball(ball_size.x / 2.),
        sprite: SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(0.5)),
            texture: texture_assets.ball.clone(),
            ..default()
        },
    });
}

fn ball_movement(
    mut ball_query: Query<(&mut Ball, &Collider, &mut Transform)>,
    paddle_query: Query<(&Transform, &Collider), (With<Paddle>, Without<Ball>)>,
    time: Res<Time>,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
) {
    let window = windows.get_primary().expect("No primary window found.");

    for (mut ball, collider, mut transform) in ball_query.iter_mut() {
        match ball.state {
            BallState::Glued { percentage } => {
                let (paddle_transform, paddle_collider) = paddle_query.single();

                let paddle_extents = paddle_collider.as_cuboid().unwrap().half_extents();

                transform.translation = paddle_transform.translation
                    + Vec3::new(
                        paddle_extents.x * 2. * (percentage - 0.5),
                        paddle_extents.y + collider.as_ball().unwrap().radius(),
                        0.0,
                    );
            }
            BallState::Free => {
                let move_vector = ball.direction * time.delta_seconds() * ball.speed;
                let mut destination = transform.translation + move_vector.extend(0.);

                let ball_radius = collider.as_ball().unwrap().radius();

                // Bounce off the top of the screen
                if destination.y + ball_radius > window.height() / 2.0 {
                    ball.direction.y = -ball.direction.y;
                    destination.y = window.height() / 2.0 - ball_radius;
                }

                // Bounce off the sides of the screen
                if destination.x.abs() > window.width() / 2.0 - ball_radius {
                    destination.x = destination.x.clamp(
                        -window.width() / 2.0 + ball_radius,
                        window.width() / 2.0 - ball_radius,
                    );
                    ball.direction.x = -ball.direction.x;
                }

                // Bounce off the paddle
                if let Some((entity, hit)) = rapier_context.cast_shape(
                    transform.translation.truncate(),
                    0.,
                    move_vector,
                    collider,
                    1.,
                    QueryFilter::default().predicate(&|entity| paddle_query.get(entity).is_ok()),
                ) {
                    // Find the collision point
                    let collision_point = transform.translation.truncate() + move_vector * hit.toi;

                    // Find the paddle's position and size
                    let (paddle_transform, paddle_collider) = paddle_query.get(entity).unwrap();
                    let paddle_center = paddle_transform.translation.truncate();
                    let paddle_extents = paddle_collider.as_cuboid().unwrap().half_extents();

                    // Make sure the ball is above the paddle
                    if collision_point.y >= paddle_center.y + paddle_extents.y {
                        // Find the percentage of the paddle that the ball hit
                        let percentage = (collision_point.x - paddle_center.x) / paddle_extents.x;

                        // Bounce the ball in the correct direction
                        ball.direction = Vec2::new(percentage / 2., 1.0).normalize();

                        // Move the ball to the correct position
                        destination = Vec3::new(collision_point.x, paddle_center.y + paddle_extents.y + ball_radius + 1., 0.0);
                    }
                }

                transform.translation = destination;
            }
        }
    }
}

fn ball_control(mut ball_query: Query<&mut Ball>, actions: Res<Actions>) {
    for mut ball in ball_query.iter_mut() {
        if actions.primary_action {
            ball.state = BallState::Free;
        }
    }
}
