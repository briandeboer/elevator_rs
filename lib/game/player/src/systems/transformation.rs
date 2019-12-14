use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::Camera,
};

use crate::components::Player;
use person::components::{Person, PersonState};

pub struct CameraTransformationSystem;

const CAMERA_MOVE_FACTOR: f32 = 35.;

impl<'s> System<'s> for CameraTransformationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Person>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, persons, players, cameras, mut transforms) = data;
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            for (_, person, _player) in (&entities, &persons, &players).join() {
                if person.state != PersonState::Jumping && person.state != PersonState::Ducking {
                    // move towards it but not too fast
                    let current_translation = transform.translation();
                    let new_y = current_translation.y
                        + (person.position.y - current_translation.y) / CAMERA_MOVE_FACTOR;
                    transform.set_translation_y(new_y);
                }
                // for now we only support one player at a time
                break;
            }
        }
    }
}
