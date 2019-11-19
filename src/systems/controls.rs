use amethyst::core::{SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Collider, Direction, Directions, Player, PlayerState};

#[derive(SystemDesc)]
pub struct ControlsSystem;

// const JUMP_DELAY: f32 = 0.2;

impl<'s> System<'s> for ControlsSystem {
    type SystemData = (
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (colliders, mut directions, mut players, input): Self::SystemData) {
        for (collider, direction, player) in
            (&colliders, &mut directions, &mut players).join()
        {
            let move_input = input.axis_value("move").expect("Move action exists");
            let jump_input = input.action_is_down("jump").expect("Jump action exists");
            let _shoot_input = input.action_is_down("shoot").expect("Shoot action exists");
            let duck_input = input.action_is_down("duck").expect("Duck action exists");

            if !duck_input {
                if player.is_ducking {
                    player.is_ducking = false;
                }
            }
            
            if move_input > 0. {
                direction.x = Directions::Right;
            } else if move_input < 0. {
                direction.x = Directions::Left;
            }
            
            player.state = if jump_input {
                PlayerState::Jumping
            } else if collider.on_ground {
                if duck_input {
                    if !player.is_ducking {
                        player.is_ducking = true;
                    }
                    PlayerState::Ducking
                } else if move_input != 0. {
                    PlayerState::Walking
                } else {
                    PlayerState::Idling
                }
            } else if player.state == PlayerState::Jumping {
                PlayerState::Jumping
            } else {
                // should be falling
                PlayerState::Idling
            }
        }
    }
}
