use amethyst::{
    assets::{Handle, Prefab},
    core::{math::Vector2, Transform, WithNamed},
    ecs::{Builder, World, WorldExt},
};

use crate::components::{
    Animation, AnimationId, AnimationPrefabData, Child, Collidee, Collider, Direction, Directions, Door, DoorEntry,
};

pub fn load_door(
    world: &mut World,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    position: Vector2<f32>,
    can_user_enter: bool,
) {
    let collider = Collider::new(16., 28.);
    let mut transform = Transform::default();
    // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
    transform.set_translation_xyz(position.x + 8., position.y - 14., 0.);
    let door = Door::new(Vector2::new(position.x, position.y), can_user_enter);
    println!(
        "Loading door! {:?}, can_user_enter: {}",
        position, can_user_enter
    );
    let animation_id = if can_user_enter {
        AnimationId::RedDoor
    } else {
        AnimationId::BlueDoor
    };
    // determine the direction based on the side
    let direction = if position.x > 104.0 {
        Directions::Right
    } else {
        Directions::Left
    };
    let door_entity = world
        .create_entity()
        .named("Door")
        .with(door)
        .with(collider)
        .with(Collidee::default())
        .with(transform)
        .with(Animation::new(
            animation_id,
            vec![
                AnimationId::RedDoor,
                AnimationId::RedDoorClose,
                AnimationId::RedDoorOpen,
                AnimationId::BlueDoor,
                AnimationId::BlueDoorOpen,
                AnimationId::BlueDoorClose,
            ],
        ))
        .with(prefab_handle.clone())
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            direction,
            Directions::Neutral,
        ))
        .build();

    if can_user_enter {
        let entry_collider = Collider::new(16., 28.);
        let mut entry_transform = Transform::default();
        // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
        entry_transform.set_translation_xyz(position.x - 1., position.y - 29., 0.);
        world
            .create_entity()
            .named("DoorEntry")
            .with(Child::new(door_entity, -1., -29., 0.))
            .with(DoorEntry::default())
            .with(entry_collider)
            .with(Collidee::default())
            .with(entry_transform)
            .with(Animation::new(
                AnimationId::DoorEntry,
                vec![
                    AnimationId::DoorEntry,
                ],
            ))
            .with(prefab_handle)
            .with(Direction::new(
                Directions::Right,
                Directions::Neutral,
                direction,
                Directions::Neutral,
            ))
            .build();

    }
}
