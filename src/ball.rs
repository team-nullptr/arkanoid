use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    actions::Actions,
    assets::TextureAssets,
    block::Block,
    paddle::{Paddle, PaddleSystem, PADDLE_ALTITUDE},
    util::cleanup,
    GameState,
};

pub const DEFAULT_BALL_SPEED: f32 = 300.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockHitEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(ball_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(ball_movement)
                    .with_system(lose_condition)
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BlockHitEvent(pub Entity);

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
    paddle_query: Query<(&Transform, &Collider), (With<Paddle>, Without<Block>, Without<Ball>)>,
    block_query: Query<(&Transform, &Collider), (With<Block>, Without<Paddle>, Without<Ball>)>,
    time: Res<Time>,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    mut hit_block_event_writer: EventWriter<BlockHitEvent>,
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

                // Utility to cast the collider
                let check_collider = |filter: QueryFilter| {
                    rapier_context.cast_shape(
                        transform.translation.truncate(),
                        0.,
                        move_vector,
                        collider,
                        1.,
                        filter,
                    )
                };

                // Bounce off the paddle
                if let Some((entity, hit)) = check_collider(
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
                        destination = Vec3::new(
                            collision_point.x,
                            paddle_center.y + paddle_extents.y + ball_radius + 1.,
                            0.0,
                        );
                    }
                }

                // Bounce off the block
                if let Some((entity, hit)) = check_collider(
                    QueryFilter::default().predicate(&|entity| block_query.get(entity).is_ok()),
                ) {
                    // Get the collision point
                    let collision_point = transform.translation.truncate() + move_vector * hit.toi;

                    // Get position of the block and it's size
                    let (block_transform, block_collider) = block_query.get(entity).unwrap();
                    let block_center = block_transform.translation.truncate();
                    let block_extents = block_collider.as_cuboid().unwrap().half_extents();

                    // Handle y bounce
                    if collision_point.y <= block_center.y - block_extents.y
                        || collision_point.y >= block_center.y + block_extents.y
                    {
                        // Calculate the direction
                        ball.direction = Vec2::new(ball.direction.x, -ball.direction.y).normalize();

                        // Move the ball
                        destination = Vec3::new(
                            collision_point.x,
                            collision_point.y + ball.direction.y / ball.direction.y.abs(),
                            0.0,
                        );
                    } else if collision_point.x <= block_center.x - block_extents.x
                        || collision_point.x >= block_center.x + block_extents.x
                    {
                        // Calculate the bounce direction
                        ball.direction = Vec2::new(-ball.direction.x, ball.direction.y).normalize();

                        // Move the ball
                        destination = Vec3::new(
                            collision_point.x + ball.direction.x / ball.direction.x.abs(),
                            collision_point.y,
                            0.,
                        );
                    }

                    // Send out the hit event
                    hit_block_event_writer.send(BlockHitEvent(entity));
                }

                transform.translation = destination;
            }
        }
    }
}

fn lose_condition(mut state: ResMut<State<GameState>>, ball_query: Query<&Transform, With<Ball>>) {
    let transform = ball_query.single();

    if transform.translation.y < PADDLE_ALTITUDE - 50. {
        if state.set(GameState::Menu).is_err() {}
    }
}

fn ball_control(mut ball_query: Query<&mut Ball>, actions: Res<Actions>) {
    for mut ball in ball_query.iter_mut() {
        if actions.primary_action {
            ball.state = BallState::Free;
        }
    }
}
