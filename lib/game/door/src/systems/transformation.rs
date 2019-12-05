use amethyst::{
    core::Transform,
    ecs::{Entities, Join, System, WriteStorage},
};

use crate::components::{Door, DoorState};
use physics::components::Collider;

pub struct DoorTransformationSystem;

impl<'s> System<'s> for DoorTransformationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Door>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut doors, mut colliders, mut transforms) = data;

        for (_entity, door, collider, transform) in
            (&entities, &mut doors, &mut colliders, &mut transforms).join()
        {
            match door.state {
                DoorState::Open => {
                    transform.set_translation_z(1.5);
                    collider.is_collidable = true;
                }
                _ => {
                    transform.set_translation_z(0.);
                    collider.is_collidable = false;
                }
            }
        }
    }
}
