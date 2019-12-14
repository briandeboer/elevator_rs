use amethyst::{
    animation::AnimationControlSet,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{BulletImpact, Gun, GunState, Person, PersonState};
use animation::components::{Animation, AnimationId};

pub struct BulletImpactAnimationSystem;

impl<'s> System<'s> for BulletImpactAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BulletImpact>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, bullet_impacts, mut animations, mut animation_control_sets) = data;

        for (entity, _, mut animation, animation_control_set) in (
            &entities,
            &bullet_impacts,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            if animation.show {
                animation_control_set.start(animation.current);
                animation.show = false;
            } else {
                let bullet_impact_animation = animation_control_set
                    .animations
                    .iter()
                    .find(|(id, _)| *id == AnimationId::BulletImpact);

                if bullet_impact_animation.is_none() {
                    let _ = entities.delete(entity);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct GunAnimationSystem;

impl<'s> System<'s> for GunAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Gun>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, guns, mut animations, mut animation_control_sets) = data;

        for (_, gun, mut animation, animation_control_set) in (
            &entities,
            &guns,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match gun.state {
                GunState::Shooting => AnimationId::PersonShoot,
                GunState::JumpShooting => AnimationId::PersonJumpShoot,
                _ => AnimationId::Holster,
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

#[derive(Default)]
pub struct PersonAnimationSystem;

impl<'s> System<'s> for PersonAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Person>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, persons, mut animations, mut animation_control_sets) = data;

        for (_, person, mut animation, animation_control_set) in (
            &entities,
            &persons,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match person.state {
                PersonState::Hopping => AnimationId::Hop,
                PersonState::Jumping => AnimationId::Jump,
                PersonState::Walking => AnimationId::Walk,
                PersonState::Shooting => AnimationId::Shoot,
                PersonState::Dying => AnimationId::Die,
                PersonState::Ducking => AnimationId::Duck,
                _ => AnimationId::Idle,
            };

            // If the new AnimationId is different to the current one, abort the
            // current animation and start the new one
            if animation.current != new_animation_id {
                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);

                animation.current = new_animation_id;
            } else if new_animation_id == AnimationId::Die {
                animation.show = false;
            }
        }
    }
}
