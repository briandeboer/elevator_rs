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

const ELEVATOR_Z: f32 = 0.0;
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
    top_left: Vector2<f32>,
    _bottom_right: Vector2<f32>,
    min_floor: usize,
    max_floor: usize,
    start_floor: usize,
) {
    // parent component
    let mut transform = Transform::default();
    transform.set_translation_xyz(top_left.x, top_left.y, ELEVATOR_Z);
    let elevator = Elevator::new(
        Vector2::new(top_left.x, top_left.y),
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

    // loop through each floor
    for i in (min_floor..=max_floor).rev() {
        let mut shaft_transform = Transform::default();
        let y = top_left.y - 48.0 * (max_floor - i) as f32 - 4.0; // FIXME: not sure why, but something is off (sprite height?)
        shaft_transform.set_translation_xyz(top_left.x, y, ELEVATOR_Z - 0.1);
        let shaft_sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: if i == min_floor { 5 } else { 3 },
        };
        // load the elevator shaft
        world
            .create_entity()
            .named("ElevatorShaft")
            .with(shaft_sprite)
            .with(shaft_transform)
            .build();
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
        world,
        elevator_entity,
        top_left,
        inside,
        sprite_sheet_handle.clone(),
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
        world,
        elevator_entity,
        top_left,
        bottom,
        sprite_sheet_handle.clone(),
    );

    let top = ElevatorComponent::new(
        "ElevatorTop",
        7,
        24.,
        4.,
        Vector3::new(0., ELEVATOR_OFFSET, 0.),
        true,
    );
    create_elevator_component(world, elevator_entity, top_left, top, sprite_sheet_handle);
}
