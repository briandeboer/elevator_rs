use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::{Builder, Entity, World, WorldExt},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
};

use crate::components::{
    Child, Collidee, Collider, Elevator, ElevatorComponent, Motion, Proximity,
};

const ELEVATOR_START_X: f32 = 116.0;
const ELEVATOR_START_Y: f32 = 70.0; // 166.0; //70.0;
const ELEVATOR_START_Z: f32 = 0.;
const ELEVATOR_OFFSET: f32 = 24.;

fn create_elevator_component(
    world: &mut World,
    elevator_entity: Entity,
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
        ELEVATOR_START_X + component.offsets.x,
        ELEVATOR_START_Y + component.offsets.y,
    );
    bbox.old_position = bbox.position;
    transform.set_translation_z(ELEVATOR_START_Z + component.offsets.z);
    let offsets = component.offsets;
    let _entity = world
        .create_entity()
        .named(component.name)
        .with(component)
        .with(Child::new(
            elevator_entity,
            ELEVATOR_START_X + offsets.x,
            ELEVATOR_START_Y + offsets.y,
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

pub fn load_elevator(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // parent component
    let mut transform = Transform::default();
    transform.set_translation_xyz(ELEVATOR_START_X, ELEVATOR_START_Y, ELEVATOR_START_Z);
    let num_floors: usize = 3;
    let start_floor: usize = 15;
    let elevator = Elevator::new(
        Vector2::new(ELEVATOR_START_X, ELEVATOR_START_Y),
        num_floors,
        start_floor,
        start_floor as f32,
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
    create_elevator_component(world, elevator_entity, inside, sprite_sheet_handle.clone());

    let bottom = ElevatorComponent::new(
        "ElevatorBottom",
        0,
        24.,
        4.,
        Vector3::new(0., -ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(world, elevator_entity, bottom, sprite_sheet_handle.clone());

    let top = ElevatorComponent::new(
        "ElevatorTop",
        3,
        24.,
        4.,
        Vector3::new(0., ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(world, elevator_entity, top, sprite_sheet_handle);
}
