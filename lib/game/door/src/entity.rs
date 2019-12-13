use amethyst::{
    assets::{Handle, Prefab},
    core::{math::Vector2, Named, Transform},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
};

use crate::components::{Door, DoorEntry, Room};
use animation::components::{Animation, AnimationId, AnimationPrefabData};
use floors::Floor;
use hierarchy::components::Child;
use physics::components::{Collidee, Collider, Direction, Directions, Motion};

pub fn load_door(
    id: usize,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    position: Vector2<f32>,
    name: &str,
    floors_overlapped: &Vec<usize>,
) {
    let can_user_enter = name == "red_left" || name == "red_right";
    let mut collider = Collider::new(4., 28.); // door is narrower for collision sake t
    collider.is_collidable = false;
    collider.bounding_box.position.x = position.x; // adjust it slightly to prevent people walking past
    collider.bounding_box.position.y = position.y;
    let mut transform = Transform::default();
    // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
    transform.set_translation_xyz(position.x, position.y, 0.25);
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
    let direction = if name == "blue_right" || name == "red_right" {
        Directions::Right
    } else {
        Directions::Left
    };

    let door_entity: Entity = entities.create();
    lazy_update.insert(door_entity, Named::new("Door"));
    lazy_update.insert(door_entity, door);
    lazy_update.insert(door_entity, collider);
    lazy_update.insert(door_entity, Collidee::default());
    lazy_update.insert(door_entity, transform);
    // add motion so that collisions will occur
    lazy_update.insert(door_entity, Motion::new());
    lazy_update.insert(
        door_entity,
        Animation::new(
            animation_id,
            vec![
                AnimationId::RedDoor,
                AnimationId::RedDoorClose,
                AnimationId::RedDoorOpen,
                AnimationId::BlueDoor,
                AnimationId::BlueDoorOpen,
                AnimationId::BlueDoorClose,
            ],
        ),
    );
    lazy_update.insert(door_entity, prefab_handle.clone());
    lazy_update.insert(
        door_entity,
        Direction::new(
            Directions::Right,
            Directions::Neutral,
            direction,
            Directions::Neutral,
        ),
    );
    lazy_update.insert(door_entity, Floor::new(vec![id], floors_overlapped.clone()));

    let mut room_transform = Transform::default();
    // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
    room_transform.set_translation_xyz(position.x, position.y, -0.25);
    let room_entity: Entity = entities.create();
    lazy_update.insert(room_entity, Named::new("Room"));
    lazy_update.insert(room_entity, Room::default());
    lazy_update.insert(room_entity, Child::new(door_entity, 0., 0., -0.1));
    lazy_update.insert(room_entity, room_transform);
    lazy_update.insert(
        room_entity,
        Animation::new(AnimationId::PurpleRoom, vec![AnimationId::PurpleRoom]),
    );
    lazy_update.insert(room_entity, prefab_handle.clone());
    lazy_update.insert(room_entity, Floor::new(vec![id], floors_overlapped.clone()));

    if can_user_enter {
        let door_entry_entity: Entity = entities.create();
        let mut entry_collider = Collider::new(1., 2.);
        let x: f32 = position.x - 9.;
        let y: f32 = position.y - 15.;
        entry_collider.bounding_box.position.x = x + 4.;
        entry_collider.bounding_box.position.y = y + 1.;
        let mut entry_transform = Transform::default();
        // position in tilesheet is based on corner not middle, remember y is reversed (bottom to top)
        entry_transform.set_translation_xyz(x, y, 0.);
        lazy_update.insert(door_entry_entity, Named::new("DoorEntry"));
        lazy_update.insert(door_entry_entity, Child::new(door_entity, x, y, 0.));
        lazy_update.insert(door_entry_entity, DoorEntry::default());
        lazy_update.insert(door_entry_entity, entry_collider);
        lazy_update.insert(door_entry_entity, Collidee::default());
        lazy_update.insert(door_entry_entity, entry_transform);
        lazy_update.insert(
            door_entry_entity,
            Animation::new(AnimationId::DoorEntry, vec![AnimationId::DoorEntry]),
        );
        lazy_update.insert(door_entry_entity, prefab_handle);
        lazy_update.insert(
            door_entry_entity,
            Direction::new(
                Directions::Right,
                Directions::Neutral,
                direction,
                Directions::Neutral,
            ),
        );
        lazy_update.insert(
            door_entry_entity,
            Floor::new(vec![id], floors_overlapped.clone()),
        );
    }
}
