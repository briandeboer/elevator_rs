use amethyst::{
    assets::{Handle, Prefab},
    core::math::{Vector2},
    core::{WithNamed},
    core::transform::Transform,
    ecs::prelude::*,
    prelude::Builder,
};

use crate::components::{
    Animation,
    AnimationId,
    AnimationPrefabData,
    Collider,
    Collidee,
    Direction,
    Directions,
    Motion,
    Player,
    PLAYER_WIDTH,
};

use crate::states::{GAME_HEIGHT, GAME_WIDTH};

/// Initialises one player in the middle-ish space
pub fn load_player(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>) {
    let mut transform = Transform::default();

    // Correctly position the player in the middle for now.
    let x = 40.0;
    let y = 200.0;
    transform.set_translation_z(0.5);

    let motion = Motion::new();

    let mut collider = Collider::new(12., 24.);
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(x + bbox.half_size.x, y - bbox.half_size.y);
    bbox.old_position = bbox.position;

    // Create a player entity.
    world
        .create_entity()
        .named("Player")
        .with(Player::new())
        .with(collider)
        .with(Collidee::default())
        .with(transform)
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Die,
                AnimationId::Jump,
                AnimationId::Idle,
                AnimationId::Walk,
                AnimationId::Duck,
            ],
        ))
        .with(prefab)
        .with(motion)
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ))
        .build();
}
