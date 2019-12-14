use amethyst::core::timing::Time;
use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::Player;
use hierarchy::components::Child;
use person::components::{Gun, GunState, Person, PersonState};
use physics::components::{Collider, Direction, Directions, Proximity};

#[derive(SystemDesc)]
pub struct PlayerGunControlsSystem;

impl<'s> System<'s> for PlayerGunControlsSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Direction>,
        ReadStorage<'s, Person>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Gun>,
        ReadStorage<'s, Child>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut directions, persons, players, mut guns, children, input, time) = data;

        // TODO: remove the maybe's and break this up
        for (direction, gun, child) in (&mut directions, &mut guns, &children).join() {
            // check if this is a player gun
            for (entity, _player, _person) in (&entities, &players, &persons).join() {
                if entity == child.parent {
                    let move_input = input.axis_value("move").expect("Move action exists");
                    let shoot_input = input.action_is_down("shoot").expect("Shoot action exists");

                    gun.state = if shoot_input && !gun.last_shoot_state && gun.shots_fired < 3 {
                        gun.last_shot_seconds = time.absolute_time_seconds();
                        GunState::Shooting
                    } else if (time.absolute_time_seconds() - gun.last_shot_seconds) < 0.05 {
                        GunState::Shooting
                    } else {
                        gun.spawned_bullet = false;
                        GunState::Holstered
                    };
                    gun.last_shoot_state = shoot_input;
                    if move_input > 0. {
                        direction.x = Directions::Right;
                    } else if move_input < 0. {
                        direction.x = Directions::Left;
                    }
                }
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct PlayerControlsSystem;

impl<'s> System<'s> for PlayerControlsSystem {
    type SystemData = (
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Direction>,
        ReadStorage<'s, Proximity>,
        WriteStorage<'s, Person>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (colliders, mut directions, proximities, mut persons, mut players, input) = data;

        // TODO: remove the maybe's and break this up
        for (collider, direction, proximity, person, player) in (
            &colliders,
            &mut directions,
            &proximities,
            &mut persons,
            &mut players,
        )
            .join()
        {
            let move_input = input.axis_value("move").expect("Move action exists");
            let jump_input = input.action_is_down("jump").expect("Jump action exists");
            let down_input = input.action_is_down("down").expect("Down action exists");

            // No changing directions when you hop
            if person.state != PersonState::Hopping {
                if move_input > 0. {
                    direction.x = Directions::Right;
                } else if move_input < 0. {
                    direction.x = Directions::Left;
                }
            }

            if !down_input && player.is_ducking {
                player.is_ducking = false;
            }

            person.state = if jump_input && !player.last_jump_state {
                PersonState::Jumping
            } else if collider.on_ground {
                if down_input && !collider.on_elevator {
                    if !player.is_ducking {
                        player.is_ducking = true;
                    }
                    PersonState::Ducking
                } else if move_input != 0. {
                    // check for close proximity of other things
                    let mut hopping = false;
                    for details in &proximity.details {
                        if details.approaching
                            && (details.other_name == "ElevatorTop"
                                || details.other_name == "ElevatorBottom"
                                || (details.other_name == "floor" && collider.on_elevator))
                        {
                            hopping = true;
                        }
                    }
                    if hopping {
                        PersonState::Hopping
                    } else {
                        PersonState::Walking
                    }
                } else {
                    PersonState::Idling
                }
            } else if person.state == PersonState::Jumping || person.state == PersonState::Hopping {
                // don't change anything until he hits the ground
                person.state
            } else {
                // should be falling
                PersonState::Idling
            };
            player.last_jump_state = jump_input;
        }
    }
}
