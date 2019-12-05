use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use components::{
    Animation, AnimationId, BulletImpact, Door, DoorState, Gun, GunState, Player, PlayerState,
};

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
                            AnimationId::PlayerShoot
                            | AnimationId::Idle
                            | AnimationId::BulletImpact => EndControl::Stay,
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

#[derive(Default)]
pub struct PlayerAnimationSystem;

impl<'s> System<'s> for PlayerAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, players, mut animations, mut animation_control_sets) = data;

        for (_, player, mut animation, animation_control_set) in (
            &entities,
            &players,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match player.state {
                PlayerState::Hopping => AnimationId::Hop,
                PlayerState::Jumping => AnimationId::Jump,
                PlayerState::Walking => AnimationId::Walk,
                PlayerState::Shooting => AnimationId::Shoot,
                PlayerState::Dying => AnimationId::Die,
                PlayerState::Ducking => AnimationId::Duck,
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
                GunState::Shooting => AnimationId::PlayerShoot,
                GunState::JumpShooting => AnimationId::PlayerJumpShoot,
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
