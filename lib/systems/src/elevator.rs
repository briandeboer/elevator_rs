use amethyst::core::timing::Time;
use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use components::{Child, Elevator, ElevatorComponent, ElevatorState, Motion};

const WAIT_TIME: f64 = 2.0;
const VELOCITY: f32 = 20.0;

#[derive(SystemDesc)]
pub struct ElevatorControlSystem;

impl<'s> System<'s> for ElevatorControlSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Elevator>,
        ReadStorage<'s, ElevatorComponent>,
        ReadStorage<'s, Child>,
        WriteStorage<'s, Motion>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut elevators, components, children, mut motions, input, time) = data;
        let up_input = input.action_is_down("up").expect("Up action exists");
        let down_input = input.action_is_down("down").expect("Down action exists");
        for (entity, elevator) in (&entities, &mut elevators).join() {
            let current_time: f64 = time.absolute_time_seconds();
            if current_time - elevator.wait_seconds > WAIT_TIME
                && elevator.state == ElevatorState::Waiting
            {
                // set it to the last known state
                elevator.state = if elevator.previous_state == ElevatorState::Waiting {
                    ElevatorState::Down
                } else {
                    elevator.previous_state
                };

                // check for boundaries
                if elevator.state == ElevatorState::Up
                    && elevator.current_floor
                        == (elevator.start_floor + elevator.num_floors - 1) as f32
                {
                    elevator.state = ElevatorState::Down;
                } else if elevator.state == ElevatorState::Down
                    && elevator.current_floor == elevator.start_floor as f32
                {
                    elevator.state = ElevatorState::Up;
                }
            } else if down_input
                && (elevator.current_floor > elevator.start_floor as f32 || elevator.velocity > 0.)
            {
                elevator.state = ElevatorState::Down;
            } else if up_input
                && elevator.current_floor < (elevator.start_floor + elevator.num_floors) as f32
            {
                elevator.state = ElevatorState::Up;
            }

            elevator.velocity = match elevator.state {
                ElevatorState::Up => VELOCITY,
                ElevatorState::Down => -VELOCITY,
                _ => 0.,
            };

            // loop through components and set their velocities
            for (_components, child, motion) in (&components, &children, &mut motions).join() {
                if child.parent == entity {
                    motion.velocity.y = elevator.velocity;
                }
            }
        }
    }
}
