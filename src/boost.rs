use crate::{assets::TextureAssets, paddle::Paddle, GameState};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_rapier2d::prelude::*;

// This could be but inside of a boost or a boost type later.
pub const DEFAULT_BOOST_SPEED: f32 = 150.;

pub struct BoostPlugin;

impl Plugin for BoostPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(boost_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(boost_movement)
                    .with_system(boost_picker),
            );
    }
}

#[derive(Component)]
pub struct Boost;

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
            boost: Boost,
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

fn boost_movement(mut boost_query: Query<&mut Transform, With<Boost>>, time: Res<Time>) {
    for mut transform in boost_query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * DEFAULT_BOOST_SPEED;
    }
}

// This probably can be done easier with rapier events or something
// FIXME: Look into rapier things
fn boost_picker(
    paddle_query: Query<(&Collider, &Transform), (With<Paddle>, Without<Boost>)>,
    boost_query: Query<(&Collider, &Transform), (With<Boost>, Without<Paddle>)>,
) {
    let (paddle_collider, paddle_transform) = paddle_query.get_single().expect("Paddle not found");

    for (boost_collider, boost_transform) in boost_query.iter() {
        if collide(
            boost_transform.translation,
            boost_collider.as_cuboid().unwrap().half_extents(),
            paddle_transform.translation,
            paddle_collider.as_cuboid().unwrap().half_extents(),
        ).is_some() {
            println!("boost picked");
        }
    }
}
