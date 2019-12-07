use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::{Builder, Entity, World, WorldExt},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
};

use crate::components::{Elevator, ElevatorComponent};
use hierarchy::components::Child;
use physics::components::{Collidee, Collider, Motion, Proximity};

const ELEVATOR_Z: f32 = 0.;
const ELEVATOR_OFFSET: f32 = 24.;

fn create_elevator_component(
    world: &mut World,
    elevator_entity: Entity,
    position: Vector2<f32>,
    component: ElevatorComponent,
    sprite_sheet_handle: SpriteSheetHandle,
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
    let _entity = world
        .create_entity()
        .named(component.name)
        .with(component)
        .with(Child::new(
            elevator_entity,
            position.x + offsets.x,
            position.y + offsets.y,
            offsets.z,
        ))
        .with(collider)
        .with(Collidee::default())
        .with(Motion::new())
        .with(Proximity::default())
        .with(render)
        .with(transform)
        .build();
}

pub fn load_elevator(
    world: &mut World,
    sprite_sheet_handle: SpriteSheetHandle,
    position: Vector2<f32>,
    min_floor: usize,
    max_floor: usize,
    start_floor: usize,
) {
    // parent component
    let mut transform = Transform::default();
    transform.set_translation_xyz(position.x, position.y, ELEVATOR_Z);
    let elevator = Elevator::new(
        Vector2::new(position.x, position.y),
        min_floor,
        max_floor,
        start_floor,
        0.,
    );
    let elevator_entity = world
        .create_entity()
        .named("Elevator")
        .with(elevator)
        .with(Collidee::default())
        .with(transform)
        .build();

    let inside = ElevatorComponent::new(
        "ElevatorInside",
        2,
        24.,
        40.,
        Vector3::new(0., 0., 0.),
        false,
    );
    create_elevator_component(world, elevator_entity, position, inside, sprite_sheet_handle.clone());

    let bottom = ElevatorComponent::new(
        "ElevatorBottom",
        0,
        24.,
        4.,
        Vector3::new(0., -ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(world, elevator_entity, position, bottom, sprite_sheet_handle.clone());

    let top = ElevatorComponent::new(
        "ElevatorTop",
        3,
        24.,
        4.,
        Vector3::new(0., ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(world, elevator_entity, position, top, sprite_sheet_handle);
}
