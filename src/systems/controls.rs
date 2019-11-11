use amethyst::core::{timing::Time, SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Direction, Directions, Player, PlayerState};

#[derive(SystemDesc)]
pub struct ControlsSystem;

const SPEED: f32 = 50.0;
const DUCK_OFFSET: f32 = 5.0;
const JUMP_OFFSET: f32 = 12.0;
const JUMP_TIME: f32 = 0.55;
const JUMP_DELAY: f32 = 0.2;

impl<'s> System<'s> for ControlsSystem {
    type SystemData = (
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut directions, mut players, input, mut transforms, time): Self::SystemData) {
        for (direction, player, transform) in (&mut directions, &mut players, &mut transforms).join() {
            let move_input = input.axis_value("move").expect("Move action exists");
            let jump_input = input.action_is_down("jump").expect("Jump action exists");
            let shoot_input = input.action_is_down("shoot").expect("Shoot action exists");
            let duck_input = input.action_is_down("duck").expect("Duck action exists");

            if !duck_input {
                if player.is_ducking {
                    transform.prepend_translation_y(DUCK_OFFSET);
                    player.is_ducking = false;
                }
            }
            
            if move_input > 0. {
                direction.x = Directions::Right;
            } else if move_input < 0. {
                direction.x = Directions::Left;
            }
            
            if let Some(t) = player.jump_time {
                let delta = t - time.delta_seconds();
                player.jump_time = if t <= 0. {
                    transform.prepend_translation_y(-JUMP_OFFSET);
                    None
                } else {
                    Some(delta)
                };
            } else {
                player.state = if jump_input {
                    player.velocity[0] = player.velocity[0] / 1.5;
                    player.jump_time = Some(JUMP_TIME);
                    transform.prepend_translation_y(JUMP_OFFSET);
                    PlayerState::Jumping
                } else if duck_input {
                    player.velocity[0] = 0.0;
                    if !player.is_ducking {
                        transform.prepend_translation_y(-DUCK_OFFSET);
                        player.is_ducking = true;
                    }
                    PlayerState::Ducking
                } else if move_input > 0. {
                    player.velocity[0] = move_input * SPEED;
                    PlayerState::Walking
                } else if move_input < 0. {
                    player.velocity[0] = move_input * SPEED;
                    PlayerState::Walking
                } else {
                    player.velocity[0] = 0.0;
                    PlayerState::Idling
                };
            };
        }
    }
}
