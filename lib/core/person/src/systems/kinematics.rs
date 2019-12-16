use amethyst::{
    core::math::Vector2,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Person, PersonState};
use physics::components::{Collider, Direction, Motion};

const GRAVITY_AMOUNT: f32 = -5.;
const FRICTION_AMOUNT: f32 = -12.0;
const WALK_ACCELERATION: f32 = 16.0;

pub struct PersonKinematicsSystem;

impl<'s> System<'s> for PersonKinematicsSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Person>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, dirs, persons, mut motions) = data;

        for (collider, dir, person, motion) in
            (&mut colliders, &dirs, &persons, &mut motions).join()
        {
            let mut acceleration = Vector2::new(0., 0.);
            match person.state {
                PersonState::Idling | PersonState::Ducking => {
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
                PersonState::Walking => {
                    acceleration = Vector2::new(WALK_ACCELERATION, GRAVITY_AMOUNT);
                }
                PersonState::Jumping => {
                    if collider.on_ground {
                        motion.velocity.y = if collider.on_elevator {
                            // for now this is a bit hacky to make sure the person doesn't overjump
                            if person.ride_velocity.y > 0. {
                                person.max_jump_velocity + person.ride_velocity.y + 5.
                            } else {
                                person.max_jump_velocity + person.ride_velocity.y / 2. - 10.
                            }
                        } else {
                            person.max_jump_velocity
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
                PersonState::Hopping => {
                    if collider.on_ground {
                        motion.velocity.y = person.max_jump_velocity / 2.;
                        collider.on_ground = false;
                    }
                    acceleration = Vector2::new(WALK_ACCELERATION, GRAVITY_AMOUNT);
                }
                PersonState::Dying => {
                    motion.velocity.x = -motion.velocity.x;
                    acceleration = Vector2::new(0., GRAVITY_AMOUNT);
                }
                _ => {}
            }
            motion.update_velocity(acceleration, dir, 0., person.max_ground_speed);
            // move faster downward when on the elevator sperson he doesn't bounce due to gravity
            if collider.on_elevator
                && (person.state == PersonState::Idling || person.state == PersonState::Walking)
            {
                motion.velocity.y = -40.;
            }
        }
    }
}
