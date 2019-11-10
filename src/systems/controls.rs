use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::Player;

#[derive(SystemDesc)]
pub struct ControlsSystem;

const SPEED: f32 = 100.0;

impl<'s> System<'s> for ControlsSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut players, input, transforms): Self::SystemData) {
        for (player, _transform) in (&mut players, &transforms).join() {
            if let Some(mv_amount) = input.axis_value("horizontal") {
                if mv_amount != 0.0 {
                    player.velocity[0] = mv_amount * SPEED;
                } else {
                    player.velocity[0] = 0.0;
                }
            }
        }
    }
}
