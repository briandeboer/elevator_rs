use amethyst::{
    core::{
        math::Vector2,
        WithNamed, Transform,
    },
    ecs::{Builder, World, WorldExt},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
};

use crate::components::{
    Collidee, Collider, Elevator, ElevatorBottom, ElevatorTop, Motion,
};

const ELEVATOR_START_X: f32 = 116.0;
const ELEVATOR_START_Y: f32 = 80.0;
const ELEVATOR_OFFSET: f32 = 24.;

pub fn load_elevator(
    world: &mut World,
    sprite_sheet_handle: SpriteSheetHandle,
) {
    // body of the elevator
    let mut transform = Transform::default();
    transform.set_translation_xyz(ELEVATOR_START_X, ELEVATOR_START_Y, 0.);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 2, // inside color
    };
    let elevator = world
        .create_entity()
        .named("Elevator")
        .with(sprite_render)
        .with(Elevator::new())
        .with(transform)
        .build();

    // top
    let top_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 3,
    };
    let mut top_transform = Transform::default();
    let mut top_collider = Collider::new(16., 8.);
    let top_bbox = &mut top_collider.bounding_box;
    top_bbox.position = Vector2::new(ELEVATOR_START_X, ELEVATOR_START_Y + ELEVATOR_OFFSET);
    top_bbox.old_position = top_bbox.position;
    top_transform.set_translation_z(0.5);
    let _top = world
        .create_entity()
        .named("ElevatorTop")
        .with(ElevatorTop::new(elevator))
        .with(top_collider)
        .with(Collidee::default())
        .with(top_render)
        .with(top_transform)
        .with(Motion::new())
        .build();

    // bottom
    let bottom_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let mut bottom_transform = Transform::default();
    let mut bottom_collider = Collider::new(16., 8.);
    let bottom_bbox = &mut bottom_collider.bounding_box;
    bottom_bbox.position = Vector2::new(ELEVATOR_START_X, ELEVATOR_START_Y - ELEVATOR_OFFSET);
    bottom_bbox.old_position = bottom_bbox.position;
    bottom_transform.set_translation_z(0.5);
    let _top = world
        .create_entity()
        .named("ElevatorBottom")
        .with(ElevatorBottom::new(elevator))
        .with(bottom_collider)
        .with(Collidee::default())
        .with(Motion::new())
        .with(bottom_render)
        .with(bottom_transform)
        .build();

}
