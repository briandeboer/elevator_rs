use amethyst::core::timing::Time;
use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{
    Collider, Direction, Directions, Gun, GunState, Player, PlayerState, Proximity,
};

#[derive(SystemDesc)]
pub struct ControlsSystem;

// const JUMP_DELAY: f32 = 0.2;

impl<'s> System<'s> for ControlsSystem {
    type SystemData = (
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Direction>,
        ReadStorage<'s, Proximity>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Gun>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (colliders, mut directions, proximities, mut players, mut guns, input, time) = data;

        for (maybe_collider, direction, maybe_proximity, maybe_player, maybe_gun) in (
            colliders.maybe(),
            &mut directions,
            proximities.maybe(),
            (&mut players).maybe(),
            (&mut guns).maybe(),
        )
            .join()
        {
            // TODO: refactor this
            if maybe_gun.is_some() || maybe_player.is_some() {
                let move_input = input.axis_value("move").expect("Move action exists");
                let jump_input = input.action_is_down("jump").expect("Jump action exists");
                let shoot_input = input.action_is_down("shoot").expect("Shoot action exists");
                let down_input = input.action_is_down("down").expect("Down action exists");

                if let Some(gun) = maybe_gun {
                    gun.state = if shoot_input && !gun.last_shoot_state && gun.shots_fired < 3 {
                        gun.last_shot_seconds = time.absolute_time_seconds();
                        GunState::Shooting
                    } else {
                        if (time.absolute_time_seconds() - gun.last_shot_seconds) < 0.05 {
                            GunState::Shooting
                        } else {
                            gun.spawned_bullet = false;
                            GunState::Holstered
                        }
                    };
                    gun.last_shoot_state = shoot_input;
                    if move_input > 0. {
                        direction.x = Directions::Right;
                    } else if move_input < 0. {
                        direction.x = Directions::Left;
                    }
                }

                if let Some(player) = maybe_player {
                    // No changing directions when you hop
                    if player.state != PlayerState::Hopping {
                        if move_input > 0. {
                            direction.x = Directions::Right;
                        } else if move_input < 0. {
                            direction.x = Directions::Left;
                        }
                    }

                    if let Some(collider) = maybe_collider {
                        if !down_input {
                            if player.is_ducking {
                                player.is_ducking = false;
                            }
                        }

                        player.state = if jump_input && !player.last_jump_state {
                            PlayerState::Jumping
                        } else if collider.on_ground {
                            if down_input && !collider.on_elevator {
                                if !player.is_ducking {
                                    player.is_ducking = true;
                                }
                                PlayerState::Ducking
                            } else if move_input != 0. {
                                // check for close proximity of other things
                                let mut hopping = false;
                                if let Some(proximity) = maybe_proximity {
                                    for details in &proximity.details {
                                        if details.approaching
                                            && (details.other_name == "ElevatorTop"
                                                || details.other_name == "ElevatorBottom"
                                                || (details.other_name == "floor"
                                                    && collider.on_elevator))
                                        {
                                            hopping = true;
                                        }
                                    }
                                }
                                if hopping {
                                    PlayerState::Hopping
                                } else {
                                    PlayerState::Walking
                                }
                            } else {
                                PlayerState::Idling
                            }
                        } else if player.state == PlayerState::Jumping
                            || player.state == PlayerState::Hopping
                        {
                            // don't change anything until he hits the ground
                            player.state
                        } else {
                            // should be falling
                            PlayerState::Idling
                        }
                    }
                    player.last_jump_state = jump_input;
                }
            }
        }
    }
}
