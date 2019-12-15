use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Animation, AnimationId};

#[derive(Default)]
pub struct AnimationControlSystem;

impl<'s> System<'s> for AnimationControlSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Animation>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, animations, animation_sets, mut animation_control_sets) = data;

        // Iterate over all entities having Animation and AnimationSet components.
        for (entity, animation, animation_set) in (&entities, &animations, &animation_sets).join() {
            // Fetch or create the AnimationControlSet for this entity.
            let animation_control_set =
                get_animation_set(&mut animation_control_sets, entity).unwrap();

            if animation.show {
                animation.types.iter().for_each(|&animation_id| {
                    // Add the animations to the AnimationControlSet if it doesn't exist already.
                    // This ensures they are re-added after a call to abort().
                    if !animation_control_set.has_animation(animation_id) {
                        let end = match animation_id {
                            AnimationId::PersonShoot
                            | AnimationId::Idle
                            | AnimationId::BulletImpact
                            | AnimationId::Die => EndControl::Stay,
                            _ => EndControl::Loop(None),
                        };
                        animation_control_set.add_animation(
                            animation_id,
                            &animation_set.get(&animation_id).unwrap(),
                            end,
                            1.0,
                            AnimationCommand::Init,
                        );
                    }
                });
            }

            // Start the animation for the current AnimationId
            animation_control_set.start(animation.current);
        }
    }
}
