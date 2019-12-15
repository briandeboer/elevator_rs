use amethyst::{
    assets::{Handle, Prefab},
    core::math::Vector2,
    core::transform::Transform,
    core::WithNamed,
    ecs::prelude::*,
    prelude::Builder,
};

use crate::components::Player;
use animation::components::{Animation, AnimationId, AnimationPrefabData};
use floors::Floor;
use hierarchy::components::Child;
use person::components::{Gun, Person};
use physics::components::{Collidee, Collider, Direction, Directions, Motion, Proximity};

/// Initialises one player in the middle-ish space
pub fn load_player(
    world: &mut World,
    player_prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    guns_prefab_handle: Handle<Prefab<AnimationPrefabData>>,
) {
    let mut transform = Transform::default();

    // FIXME: Set these to not be hardcoded
    // Correctly position the player in the middle for now.
    let x = 40.0;
    let y = 150.0;
    let z = 0.5;
    transform.set_translation_z(z);

    let mut collider = Collider::new(12., 24.);
    collider.allow_proximity = true;
    collider.is_person = true;
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(x + bbox.half_size.x, y - bbox.half_size.y);
    bbox.old_position = bbox.position;

    // THOUGHTS: gun should just have position offsets
    // move gun after the person moves and just adjust based on offset
    // to hide the gun just make it a sprite that you can't see because it's all transparent!
    // create the gun first
    // Create a player entity.
    let player = world
        .create_entity()
        .named("Player")
        .with(Person::new())
        .with(Player::new())
        .with(collider)
        .with(Collidee::default())
        .with(transform)
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Die,
                AnimationId::Hop,
                AnimationId::Jump,
                AnimationId::Idle,
                AnimationId::Walk,
                AnimationId::Duck,
            ],
        ))
        .with(player_prefab_handle)
        .with(Motion::new())
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ))
        .with(Proximity::default())
        .with(Floor::new(vec![0], vec![30,31]))
        .build();

    let mut gun_transform = Transform::default();
    gun_transform.set_translation_xyz(x, y, 0.7);
    world
        .create_entity()
        .named("Gun")
        .with(Child::new(player, 8., 2., 0.))
        .with(Gun::new(true))
        .with(gun_transform)
        .with(Animation::new(
            AnimationId::Holster,
            vec![
                AnimationId::PersonShoot,
                AnimationId::PersonJumpShoot,
                AnimationId::Holster,
            ],
        ))
        .with(guns_prefab_handle)
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ))
        .with(Floor::new(vec![0], vec![30,31]))
        .build();
}
