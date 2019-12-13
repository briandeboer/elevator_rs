use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Named, Transform,
    },
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
};

use crate::components::{Elevator, ElevatorComponent};
use floors::Floor;
use hierarchy::components::Child;
use physics::components::{Collidee, Collider, Motion, Proximity};

const ELEVATOR_Z: f32 = 0.0;
const ELEVATOR_OFFSET: f32 = 24.;

fn create_elevator_component(
    id: usize,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
    elevator_entity: Entity,
    position: Vector2<f32>,
    component: ElevatorComponent,
    sprite_sheet_handle: SpriteSheetHandle,
    floors_overlapped: Vec<usize>,
) {
    let render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: component.sprite_number,
    };
    let mut transform = Transform::default();
    let mut collider = Collider::new(component.width, component.height);
    collider.is_collidable = component.is_collidable;
    collider.is_rideable = true;
    collider.allow_proximity = collider.is_collidable;
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(
        position.x + component.offsets.x,
        position.y + component.offsets.y,
    );
    bbox.old_position = bbox.position;
    transform.set_translation_z(ELEVATOR_Z + component.offsets.z);
    let offsets = component.offsets;
    let entity: Entity = entities.create();
    lazy_update.insert(entity, Named::new(component.name));
    lazy_update.insert(entity, component);
    lazy_update.insert(
        entity,
        Child::new(
            elevator_entity,
            position.x + offsets.x,
            position.y + offsets.y,
            offsets.z,
        ),
    );
    lazy_update.insert(entity, collider);
    lazy_update.insert(entity, Collidee::default());
    lazy_update.insert(entity, Motion::new());
    lazy_update.insert(entity, Proximity::default());
    lazy_update.insert(entity, render);
    lazy_update.insert(entity, transform);
    lazy_update.insert(entity, Floor::new(vec![id], floors_overlapped));
}

pub fn load_elevator(
    id: usize,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
    sprite_sheet_handle: SpriteSheetHandle,
    top_left: Vector2<f32>,
    _bottom_right: Vector2<f32>,
    min_floor: usize,
    max_floor: usize,
    start_floor: usize,
) {
    // parent component
    let floors_overlapped: Vec<usize> = (min_floor..=max_floor).collect();
    let mut transform = Transform::default();
    transform.set_translation_xyz(top_left.x, top_left.y, ELEVATOR_Z);
    let elevator = Elevator::new(
        Vector2::new(top_left.x, top_left.y),
        min_floor,
        max_floor,
        start_floor,
        0.,
    );
    let elevator_entity: Entity = entities.create();
    lazy_update.insert(elevator_entity, Named::new("Elevator"));
    lazy_update.insert(elevator_entity, elevator);
    lazy_update.insert(elevator_entity, Collidee::default());
    lazy_update.insert(elevator_entity, transform);
    lazy_update.insert(
        elevator_entity,
        Floor::new(vec![id], floors_overlapped.clone()),
    );

    // loop through each floor
    for i in (min_floor..=max_floor).rev() {
        let mut shaft_transform = Transform::default();
        let y = top_left.y - 48.0 * (max_floor - i) as f32 - 4.0; // FIXME: not sure why, but something is off (sprite height?)
        shaft_transform.set_translation_xyz(top_left.x, y, ELEVATOR_Z - 0.1);
        let shaft_sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 3,
        };
        // load the elevator shaft
        let shaft_entity: Entity = entities.create();
        lazy_update.insert(shaft_entity, Named::new("ElevatorShaft"));
        lazy_update.insert(shaft_entity, shaft_sprite);
        lazy_update.insert(shaft_entity, shaft_transform);
        lazy_update.insert(
            shaft_entity,
            Floor::new(vec![id], floors_overlapped.clone()),
        );

        let mut overlay_transform = Transform::default();
        overlay_transform.set_translation_xyz(top_left.x, y, ELEVATOR_Z + 0.1);
        let overlay_sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: if i == min_floor { 6 } else { 4 },
        };
        let overlay_entity = entities.create();
        lazy_update.insert(overlay_entity, Named::new("Overlay"));
        lazy_update.insert(overlay_entity, overlay_sprite);
        lazy_update.insert(overlay_entity, overlay_transform);
        lazy_update.insert(
            overlay_entity,
            Floor::new(vec![id], floors_overlapped.clone()),
        );
    }

    let inside = ElevatorComponent::new(
        "ElevatorInside",
        2,
        24.,
        40.,
        Vector3::new(0., 0., 0.),
        false,
    );
    create_elevator_component(
        id,
        entities,
        lazy_update,
        elevator_entity,
        top_left,
        inside,
        sprite_sheet_handle.clone(),
        floors_overlapped.clone(),
    );

    let bottom = ElevatorComponent::new(
        "ElevatorBottom",
        0,
        24.,
        4.,
        Vector3::new(0., -ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(
        id,
        entities,
        lazy_update,
        elevator_entity,
        top_left,
        bottom,
        sprite_sheet_handle.clone(),
        floors_overlapped.clone(),
    );

    let top = ElevatorComponent::new(
        "ElevatorTop",
        8,
        24.,
        4.,
        Vector3::new(0., ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(
        id,
        entities,
        lazy_update,
        elevator_entity,
        top_left,
        top,
        sprite_sheet_handle,
        floors_overlapped.clone(),
    );
}
