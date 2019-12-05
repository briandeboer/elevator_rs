use amethyst::{
    assets::{Handle, Prefab},
    core::{math::Vector2, Transform, WithNamed},
    ecs::{Builder, World, WorldExt},
};

use crate::components::{Door, DoorEntry, Room};
use animation::components::{Animation, AnimationId, AnimationPrefabData};
use hierarchy::components::Child;
use physics::components::{Collidee, Collider, Direction, Directions, Motion};

pub fn load_door(
    world: &mut World,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    position: Vector2<f32>,
    can_user_enter: bool,
) {
    let mut collider = Collider::new(4., 28.); // door is narrower for collision sake t
    collider.is_collidable = false;
    collider.bounding_box.position.x = position.x + 8.; // adjust it slightly to prevent people walking past
    collider.bounding_box.position.y = position.y - 14.;
    let mut transform = Transform::default();
    // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
    transform.set_translation_xyz(position.x + 8., position.y - 14., 0.1);
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
        // add motion so that collisions will occur
        .with(Motion::new())
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

    let mut room_transform = Transform::default();
    // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
    room_transform.set_translation_xyz(position.x + 8., position.y - 14., 0.);
    let _room = world
        .create_entity()
        .named("Room")
        .with(Room::default())
        .with(Child::new(door_entity, 0., 0., -0.1))
        .with(room_transform)
        .with(Animation::new(
            AnimationId::PurpleRoom,
            vec![AnimationId::PurpleRoom],
        ))
        .with(prefab_handle.clone())
        .build();

    if can_user_enter {
        let mut entry_collider = Collider::new(1., 2.);
        let x: f32 = position.x - 1.;
        let y: f32 = position.y - 29.;
        entry_collider.bounding_box.position.x = x + 4.;
        entry_collider.bounding_box.position.y = y + 1.;
        let mut entry_transform = Transform::default();
        // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
        entry_transform.set_translation_xyz(x, y, 0.);
        world
            .create_entity()
            .named("DoorEntry")
            .with(Child::new(door_entity, x, y, 0.))
            .with(DoorEntry::default())
            .with(entry_collider)
            .with(Collidee::default())
            .with(entry_transform)
            .with(Animation::new(
                AnimationId::DoorEntry,
                vec![AnimationId::DoorEntry],
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
