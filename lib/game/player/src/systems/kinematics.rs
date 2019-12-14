use amethyst::{
    core::math::Vector2,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Player, PlayerState};
use physics::components::{Collider, Direction, Motion};

const GRAVITY_AMOUNT: f32 = -5.;
const FRICTION_AMOUNT: f32 = -12.0;
const WALK_ACCELERATION: f32 = 16.0;

pub struct PlayerKinematicsSystem;

impl<'s> System<'s> for PlayerKinematicsSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, dirs, players, mut motions) = data;

        for (collider, dir, player, motion) in
            (&mut colliders, &dirs, &players, &mut motions).join()
        {
            let mut acceleration = Vector2::new(0., 0.);
            match player.state {
                PlayerState::Idling | PlayerState::Ducking => {
                    // how much skidding happens
                    let acceleration_x = if motion.velocity.x != 0. && collider.on_ground {
                        // slow down on ground when he stops
                        FRICTION_AMOUNT
                    } else if !collider.on_ground {
                        // keep moving a bit when falling
                        FRICTION_AMOUNT / 2.5
                    } else {
                        0.
                    };
                    acceleration = Vector2::new(acceleration_x, GRAVITY_AMOUNT);
                }
                PlayerState::Walking => {
                    acceleration = Vector2::new(WALK_ACCELERATION, GRAVITY_AMOUNT);
                }
                PlayerState::Jumping => {
                    if collider.on_ground {
                        motion.velocity.y = if collider.on_elevator {
                            // for now this is a bit hacky to make sure the player doesn't overjump
                            if player.ride_velocity.y > 0. {
                                player.max_jump_velocity + player.ride_velocity.y + 5.
                            } else {
                                player.max_jump_velocity + player.ride_velocity.y / 2. - 10.
                            }
                        } else {
                            player.max_jump_velocity
                        };
                        collider.on_ground = false;
                    }
                    // how much he slows down when he's in the air and not running
                    let acceleration_x = if motion.velocity.x != 0. {
                        (-WALK_ACCELERATION / 50.)
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
                    acceleration = Vector2::new(WALK_ACCELERATION, GRAVITY_AMOUNT);
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
            // move faster downward when on the elevator so that he doesn't bounce due to gravity
            if collider.on_elevator
                && (player.state == PlayerState::Idling || player.state == PlayerState::Walking)
            {
                motion.velocity.y = -40.;
            }
        }
    }
}
