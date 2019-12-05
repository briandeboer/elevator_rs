use amethyst::{
    animation::AnimationControlSet,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use animation::components::{Animation, AnimationId};
use crate::components::{Door, DoorState};

#[derive(Default)]
pub struct DoorAnimationSystem;

impl<'s> System<'s> for DoorAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Door>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, doors, mut animations, mut animation_control_sets) = data;

        for (_, door, mut animation, animation_control_set) in (
            &entities,
            &doors,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match door.state {
                DoorState::Open => {
                    if door.can_user_enter {
                        AnimationId::RedDoorOpen
                    } else {
                        AnimationId::BlueDoorOpen
                    }
                }
                DoorState::Closed => {
                    if door.can_user_enter {
                        AnimationId::RedDoor
                    } else {
                        AnimationId::BlueDoor
                    }
                }
            };

            // If the new AnimationId is different to the current one, abort the
            // current animation and start the new one
            if animation.current != new_animation_id {
                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);

                animation.current = new_animation_id;
            }
        }
    }
}
