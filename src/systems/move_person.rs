use amethyst::core::{timing::Time, SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};

use crate::components::Player;

#[derive(SystemDesc)]
pub struct MovePersonSystem;

impl<'s> System<'s> for MovePersonSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (persons, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (person, local) in (&persons, &mut locals).join() {
            local.prepend_translation_x(person.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(person.velocity[1] * time.delta_seconds());
        }
    }
}

// movement component??
// possible states
// walking, ducking, jumping,
