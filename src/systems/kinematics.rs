use amethyst::{
    core::math::Vector2,
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider, Direction, Motion, Player, PlayerState};
use crate::resources::Context;

const GRAVITY_AMOUNT: f32 = -5.;

pub struct KinematicsSystem;

impl<'s> System<'s> for KinematicsSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Motion>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, motions, time) = data;

        for (collider, motion) in (&mut colliders, &motions).join() {
            let bbox = &mut collider.bounding_box;
            bbox.old_position = bbox.position;
            bbox.position.x += motion.velocity.x * time.delta_seconds();
            bbox.position.y += motion.velocity.y * time.delta_seconds();

            let hbox = &mut collider.hit_box;
            hbox.old_position = hbox.position;
            collider.set_hit_box_position(motion.velocity);
        }
    }
}

pub struct PlayerKinematicsSystem;

impl<'s> System<'s> for PlayerKinematicsSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Motion>,
        Read<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, dirs, players, mut motions, context) = data;

        for (collider, dir, player, motion) in
            (&mut colliders, &dirs, &players, &mut motions).join()
        {
            let mut acceleration = Vector2::new(0., 0.);
            match player.state {
                PlayerState::Idling | PlayerState::Ducking => {
                    // how much skidding happens
                    let acceleration_x = if motion.velocity.x != 0. && collider.on_ground {
                        // slow down on ground when he stops
                        context.friction_amount
                    } else if !collider.on_ground {
                        // keep moving a bit when falling
                        context.friction_amount / 2.5
                    } else {
                        0.
                    };
                    acceleration = Vector2::new(acceleration_x, GRAVITY_AMOUNT);
                }
                PlayerState::Walking => {
                    acceleration = Vector2::new(context.walk_acceleration, GRAVITY_AMOUNT);
                }
                PlayerState::Jumping => {
                    if collider.on_ground {
                        motion.velocity.y = player.max_jump_velocity;
                        collider.on_ground = false;
                    }
                    // how much he slows down when he's in the air and not running
                    let acceleration_x = if motion.velocity.x != 0. {
                        (-context.walk_acceleration / 50.)
                    } else {
                        0.
                    };
                    acceleration = Vector2::new(acceleration_x, GRAVITY_AMOUNT);
                }
                PlayerState::Hopping => {
                    if collider.on_ground {
                        motion.velocity.y = player.max_jump_velocity / 2.;
                        collider.on_ground = false;
                    }
                    acceleration = Vector2::new(context.walk_acceleration, GRAVITY_AMOUNT);
                }
                // PlayerState::Dying => {
                //     if collider.on_ground {
                //         motion.velocity.x = 0.;
                //         motion.velocity.y = 8.;
                //         collider.on_ground = false;
                //     }
                //     acceleration = Vector2::new(0., GRAVITY_AMOUNT);
                // }
                _ => {}
            }
            motion.update_velocity(acceleration, dir, 0., player.max_ground_speed);
        }
    }
}
