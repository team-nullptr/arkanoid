use crate::{assets::TextureAssets, paddle::Paddle, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BoostPlugin;

impl Plugin for BoostPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyBoostEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(boost_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(boost_movement)
                    .with_system(apply_boosts),
            );
    }
}

pub enum BoostType {
    Life,
}

#[derive(Component)]
pub struct Boost {
    pub kind: BoostType,
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
                kind: BoostType::Life,
                speed: 150.,
            },
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(0., 200., 0.))
                    .with_scale(Vec3::splat(0.5)),
                ..default()
            },
            collider: Collider::cuboid(boost_size.x / 2., boost_size.y / 2.),
        }
    }
}

fn boost_setup(mut commands: Commands, textures: Res<TextureAssets>, images: Res<Assets<Image>>) {
    let boost_image = images
        .get(&textures.life_boost)
        .expect("Boost texture is not loaded");

    commands.spawn(BoostBundle::new(
        textures.life_boost.clone(),
        &boost_image.size(),
    ));
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ApplyBoostEvent {
    pub boost: BoostType,
}

fn boost_movement(
    mut commands: Commands,
    mut boost_query: Query<(&mut Transform, &Boost)>,
    time: Res<Time>,
    paddle_query: Query<(&Collider, &Transform), (With<Paddle>, Without<Boost>)>,
    rapier_context: Res<RapierContext>,
    mut apply_boost_event_writer: EventWriter<ApplyBoostEvent>,
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
        apply_boost_event_writer.send(ApplyBoostEvent {
            boost: boost_query.get(entity).unwrap().1.kind,
        });
        commands.entity(entity).despawn_recursive();
    }

    // Free fall
    for (mut transform, boost) in boost_query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * boost.speed;
    }
}

fn apply_boosts(apply_boost_event_reader: EventReader<ApplyBoostEvent>) {
    for event in apply_boost_event_reader.iter() {
        match event.boost {
            BoostType::Life => {
                println!("Life boost applied");
            }
        }
    }
}
