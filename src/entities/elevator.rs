use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::{Builder, Entity, World, WorldExt},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
};

use crate::components::{Child, Collidee, Collider, Elevator, ElevatorComponent, Motion};

const ELEVATOR_START_X: f32 = 116.0;
const ELEVATOR_START_Y: f32 = 40.0;
const ELEVATOR_START_Z: f32 = 0.;
const ELEVATOR_OFFSET: f32 = 24.;
const VELOCITY: f32 = 10.0;

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
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(
        ELEVATOR_START_X + component.offsets.x,
        ELEVATOR_START_Y + component.offsets.y,
    );
    bbox.old_position = bbox.position;
    transform.set_translation_z(ELEVATOR_START_Z + component.offsets.z);
    let mut motion = Motion::new();
    motion.velocity.y = VELOCITY;
    let collidable = component.is_collidable;
    let offsets = component.offsets;
    let entity = world
        .create_entity()
        .named(component.name)
        .with(component)
        .with(Child::new(
            elevator_entity,
            ELEVATOR_START_X + offsets.x,
            ELEVATOR_START_Y + offsets.y,
            offsets.z,
        ))
        // .with(collider)
        // .with(Collidee::default())
        .with(motion)
        .with(render)
        .with(transform)
        .build();
}

pub fn load_elevator(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // parent component
    let mut transform = Transform::default();
    transform.set_translation_xyz(ELEVATOR_START_X, ELEVATOR_START_Y, ELEVATOR_START_Z);
    let mut elevator = Elevator::default();
    elevator.velocity = VELOCITY;
    let elevator_entity = world
        .create_entity()
        .named("Elevator")
        .with(elevator)
        .with(Collidee::default())
        .with(transform)
        .build();

    let bottom = ElevatorComponent::new(
        "ElevatorBottom",
        0,
        16.,
        8.,
        Vector3::new(0., -ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(world, elevator_entity, bottom, sprite_sheet_handle.clone());

    let inside = ElevatorComponent::new(
        "ElevatorInside",
        2,
        24.,
        40.,
        Vector3::new(0., 0., 0.),
        false,
    );
    create_elevator_component(world, elevator_entity, inside, sprite_sheet_handle.clone());

    // let top = ElevatorComponent::new(
    //     "ElevatorTop",
    //     3,
    //     16.,
    //     8.,
    //     Vector3::new(0., ELEVATOR_OFFSET, 0.),
    //     true,
    // );
    // create_elevator_component(world, elevator_entity, top, sprite_sheet_handle);
}
