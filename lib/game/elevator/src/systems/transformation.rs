use amethyst::{
    core::{timing::Time, Named, Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Elevator, ElevatorComponent, ElevatorState};
use hierarchy::components::Child;
use physics::components::{Collidee, Collider, Motion};

pub struct ElevatorTransformationSystem;

fn stop_elevator(elevator: &mut Elevator, current_floor: f32, position: f32, wait_time: f64) {
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
                        // stop the elevator when necessary
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
                                        (i - 1 + elevator.start_floor) as f32,
                                        boundaries[i - 1],
                                        time.absolute_time_seconds(),
                                    );
                                } else if (elevator.state == ElevatorState::Up
                                    || elevator.state == ElevatorState::Down)
                                    && diff > 2.0
                                    && diff < 5.0
                                {
                                    // once it gets far enough away we allow it to stop
                                    elevator.can_wait = true;
                                } else if i == 1
                                    && elevator.state == ElevatorState::Down
                                    && bbox.position.y < boundaries[0]
                                {
                                    stop_elevator(
                                        elevator,
                                        elevator.start_floor as f32,
                                        boundaries[0],
                                        time.absolute_time_seconds(),
                                    );
                                } else if i == elevator.num_floors
                                    && elevator.state == ElevatorState::Up
                                    && bbox.position.y > boundaries[i - 1]
                                {
                                    stop_elevator(
                                        elevator,
                                        (i - 1 + elevator.start_floor) as f32,
                                        boundaries[i - 1],
                                        time.absolute_time_seconds(),
                                    );
                                } else if i
                                    == (elevator.current_floor.floor()
                                        - elevator.start_floor as f32
                                        + 1.0) as usize
                                {
                                    let signed_diff: f32 = bbox.position.y - boundaries[i - 1];
                                    elevator.current_floor =
                                        elevator.current_floor.floor() + signed_diff / 48.;
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
