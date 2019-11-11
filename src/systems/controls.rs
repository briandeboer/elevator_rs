use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Direction, Directions, Player, PlayerState};

#[derive(SystemDesc)]
pub struct ControlsSystem;

const SPEED: f32 = 50.0;

impl<'s> System<'s> for ControlsSystem {
    type SystemData = (
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut directions, mut players, input, transforms): Self::SystemData) {
        for (direction, player, _transform) in (&mut directions, &mut players, &transforms).join() {
            if let Some(mv_amount) = input.axis_value("horizontal") {
                if mv_amount != 0.0 {
                    player.state = PlayerState::Walking;
                    player.velocity[0] = mv_amount * SPEED;
                    if mv_amount > 0. {
                        direction.x = Directions::Right;
                    } else {
                        direction.x = Directions::Left;
                    }
                } else {
                    player.state = PlayerState::Idling;
                    player.velocity[0] = 0.0;
                }
            }
        }
    }
}
