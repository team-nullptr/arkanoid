use crate::{assets::TextureAssets, paddle::Paddle, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BoostPlugin;

impl Plugin for BoostPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(boost_setup))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(boost_movement));
    }
}

#[derive(Component)]
pub struct Boost {
    pub name: String,
    pub speed: f32,
}

#[derive(Bundle)]
pub struct BoostBundle {
    boost: Boost,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
}

impl BoostBundle {
    fn new(texture: Handle<Image>, boost_size: &Vec2) -> Self {
        Self {
            boost: Boost {
                name: "test".to_string(),
                speed: 150.,
            },
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(0., 200., 0.)),
                ..default()
            },
            collider: Collider::cuboid(boost_size.x / 2., boost_size.y / 2.),
        }
    }
}

fn boost_setup(mut commands: Commands, textures: Res<TextureAssets>, images: Res<Assets<Image>>) {
    let boost_image = images
        .get(&textures.boost)
        .expect("Boost texture is not loaded");

    commands.spawn(BoostBundle::new(
        textures.boost.clone(),
        &boost_image.size(),
    ));
}

fn boost_movement(
    mut commands: Commands,
    mut boost_query: Query<(&mut Transform, &Boost)>,
    time: Res<Time>,
    paddle_query: Query<(&Collider, &Transform), (With<Paddle>, Without<Boost>)>,
    rapier_context: Res<RapierContext>,
) {
    // Check for boost collision
    let (paddle_collider, paddle_transform) = paddle_query.single();

    if let Some((entity, _)) = rapier_context.cast_shape(
        paddle_transform.translation.truncate(),
        0.,
        Vec2::new(0., -1.),
        paddle_collider,
        1.,
        QueryFilter::default().predicate(&|entity| boost_query.get(entity).is_ok()),
    ) {
        // TODO: apply boost effect or sth
        commands.entity(entity).despawn_recursive();
    }

    // Free fall
    for (mut transform, boost) in boost_query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * boost.speed;
    }
}
