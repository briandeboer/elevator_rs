use amethyst::{
    core::{timing::Time, Named, Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Child, Collidee, Collider, Direction, Elevator, ElevatorComponent, ElevatorState, Gun, Motion,
    Player, PlayerState,
};

const SAFE_PADDING: f32 = 0.0001;

pub struct PlayerTransformationSystem;

impl<'s> System<'s> for PlayerTransformationSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut colliders, mut collidees, mut motions, mut transforms) = data;

        for (player, collider, collidee, motion, transform) in (
            &mut players,
            &mut colliders,
            &mut collidees,
            &mut motions,
            &mut transforms,
        )
            .join()
        {
            // find anything that the player is colliding with
            let bbox = &mut collider.bounding_box;
            let velocity = &mut motion.velocity;

            // if the player collides with something horizontally, he should stop moving
            // and if the thing is rideable it then the player should not be able to push past it
            if let Some(collidee_horizontal) = collidee.horizontal.take() {
                velocity.x = 0.;
                // TODO: change this to mass and velocity?
                if collidee_horizontal.is_rideable {
                    // use the correction to determine which end of the rideable and which way we are getting pushed
                    if collidee_horizontal.correction < 0. {
                        bbox.position.x = collidee_horizontal.position.x
                            + collidee_horizontal.half_size.x
                            + bbox.half_size.x
                            + SAFE_PADDING;
                    } else {
                        bbox.position.x = collidee_horizontal.position.x
                            - collidee_horizontal.half_size.x
                            - bbox.half_size.x
                            - SAFE_PADDING;
                    }
                } else {
                    bbox.position.x -= collidee_horizontal.correction;
                }
            }
            // if the player collides with something vertically he should stop moving vertically
            // if the player is on somthing rideable he should move with it
            if let Some(collidee_vertical) = collidee.vertical.take() {
                velocity.y = 0.;
                if collidee_vertical.is_rideable {
                    // use the correction to determine which end of the rideable and which way we are getting pushed
                    if collidee_vertical.correction < 0. {
                        bbox.position.y = collidee_vertical.position.y
                            + collidee_vertical.half_size.y
                            + bbox.half_size.y
                            + SAFE_PADDING;
                    } else {
                        bbox.position.y = collidee_vertical.position.y
                            - collidee_vertical.half_size.y
                            - bbox.half_size.y
                            - SAFE_PADDING;
                    }
                } else {
                    bbox.position.y -= collidee_vertical.correction;
                }

                if collidee_vertical.correction < 0. {
                    collider.on_ground = true;
                }
                if collidee_vertical.name == "ElevatorBottom" || collidee_vertical.name == "ElevatorTop" {
                    collider.on_elevator = true;
                }
            } else {
                collider.on_elevator = false;
            }

            if velocity.y != 0. {
                collider.on_ground = false;
            }

            let x = bbox.position.x;
            let mut y = bbox.position.y;

            collider.set_hit_box_position(*velocity);
            if player.state == PlayerState::Ducking {
                y -= 4.0;
            }

            player.update_position(x, y);

            transform.set_translation_x(x);
            transform.set_translation_y(y);
        }
    }
}

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Elevator>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        // let (mut players, mut elevators, mut colliders, mut collidees, mut motions, mut transforms) =
        //     data;

        // for (maybe_player, maybe_elevator, collider, collidee, motion, transform) in (
        //     (&mut players).maybe(),
        //     (&mut elevators).maybe(),
        //     &mut colliders,
        //     &mut collidees,
        //     &mut motions,
        //     &mut transforms,
        // )
        //     .join()
        // {
        //     let bbox = &mut collider.bounding_box;
        //     let velocity = &mut motion.velocity;

        //     let x = bbox.position.x;
        //     let mut y = bbox.position.y;

        //     if let Some(collidee_horizontal) = collidee.horizontal.take() {
        //         bbox.position.x -= collidee_horizontal.correction;
        //         velocity.x = 0.;
        //     }
        //     if let Some(collidee_vertical) = collidee.vertical.take() {
        //         velocity.y = 0.;
        //         if collidee_vertical.correction < 0. {
        //             collider.on_ground = true;
        //             if collidee_vertical.name == "ElevatorBottom" {
        //                 collider.on_elevator = true;
        //                 velocity.y = 1.0;
        //             } else {
        //                 collider.on_elevator = false;
        //                 bbox.position.y -= collidee_vertical.correction;
        //             }
        //         }
        //     }
        //     // FIXME: Due to the take() operation above, collidee.vertical will always be NONE.
        //     // Might indicate a bug.
        //     if velocity.y != 0. {
        //         collider.on_ground = false;
        //     }

        //     collider.set_hit_box_position(*velocity);
        //     if let Some(player) = maybe_player {
        //         if player.state == PlayerState::Ducking {
        //             y -= 4.0;
        //         }
        //         player.update_position(x, y);
        //     }

        //     // FIXME: move this
        //     // if let Some(elevator) = maybe_elevator {
        //     //     elevator.update_position(x, y);
        //     // }

        //     transform.set_translation_x(x);
        //     transform.set_translation_y(y);
        // }
    }
}

pub struct GunTransformationSystem;

impl<'s> System<'s> for GunTransformationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Gun>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, guns, directions, players, mut transforms) = data;

        // loop through all guns and get the parent entity
        for (gun, direction, transform) in (&guns, &directions, &mut transforms).join() {
            if let Some(parent) = gun.parent {
                for (entity, player) in (&entities, &players).join() {
                    if entity == parent {
                        if direction.x != direction.default_x {
                            transform.set_translation_x(player.pos_x - gun.position_offset.x);
                        } else {
                            transform.set_translation_x(player.pos_x + gun.position_offset.x);
                        }
                        transform.set_translation_y(player.pos_y + gun.position_offset.y);
                    }
                }
            }
        }
    }
}

pub struct ElevatorTransformationSystem;

fn stop_elevator(elevator: &mut Elevator, current_floor: usize, position: f32, wait_time: f64) {
    elevator.current_floor = current_floor;
    elevator.velocity = 0.;
    elevator.previous_state = elevator.state;
    elevator.state = ElevatorState::Waiting;
    elevator.wait_seconds = wait_time;
    elevator.position.y = position;
    elevator.can_wait = false;
}

impl<'s> System<'s> for ElevatorTransformationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Elevator>,
        ReadStorage<'s, ElevatorComponent>,
        ReadStorage<'s, Child>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Named>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut elevators,
            components,
            children,
            mut colliders,
            mut collidees,
            mut motions,
            mut transforms,
            names,
            time,
        ) = data;

        for (component, child, collider, _collidee, motion, transform, named) in (
            &components,
            &children,
            &mut colliders,
            &mut collidees,
            &mut motions,
            &mut transforms,
            &names,
        )
            .join()
        {
            let name = &named.name;
            let bbox = &mut collider.bounding_box;
            let velocity = &mut motion.velocity;

            let parent = child.parent;

            for (entity, elevator) in (&entities, &mut elevators).join() {
                if parent == entity {
                    let mut x = bbox.position.x;
                    let mut y = bbox.position.y;

                    if name.to_string() == "ElevatorInside" {
                        elevator.position.x = x;
                        elevator.position.y = y;
                        if elevator.state != ElevatorState::Waiting {
                            let boundaries = elevator.boundaries.clone();
                            for i in 1..=elevator.num_floors {
                                let diff = (bbox.position.y - boundaries[i - 1]).abs();
                                if (elevator.state == ElevatorState::Up
                                    || elevator.state == ElevatorState::Down)
                                    && diff < 0.5
                                    && elevator.can_wait
                                {
                                    stop_elevator(
                                        elevator,
                                        i - 1,
                                        boundaries[i - 1],
                                        time.absolute_time_seconds(),
                                    );
                                } else if (elevator.state == ElevatorState::Up
                                    || elevator.state == ElevatorState::Down)
                                    && diff > 2.0 && diff < 5.0
                                {
                                    // once it gets far enough away we allow it to stop
                                    elevator.can_wait = true;
                                } else if i == 1
                                    && elevator.state == ElevatorState::Down
                                    && bbox.position.y < boundaries[0]
                                {
                                    stop_elevator(
                                        elevator,
                                        0,
                                        boundaries[0],
                                        time.absolute_time_seconds(),
                                    );
                                } else if i == elevator.num_floors
                                    && elevator.state == ElevatorState::Up
                                    && bbox.position.y > boundaries[i - 1]
                                {
                                    stop_elevator(
                                        elevator,
                                        i - 1,
                                        boundaries[i - 1],
                                        time.absolute_time_seconds(),
                                    );
                                }
                            }
                        }
                    }

                    velocity.y = elevator.velocity;

                    // if we are at a boundary, line everything up
                    if elevator.velocity == 0. {
                        x = elevator.position.x + component.offsets.x;
                        y = elevator.position.y + component.offsets.y;
                        bbox.position.x = x;
                        bbox.position.y = y;
                        collider.hit_box.position.x = x;
                        collider.hit_box.position.y = y;
                    } else {
                        // collider.set_hit_box_position(*velocity);
                    }
                    transform.set_translation_x(x);
                    transform.set_translation_y(y);
                    // break;
                }
            }
        }
    }
}
