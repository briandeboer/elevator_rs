use amethyst::{
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{Gun, Motion};

pub struct ShootSystem;

impl<'s> System<'s> for ShootSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Gun>,
        ReadStorage<'s, Motion>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, motions, time) = data;

        for (collider, motion) in (&mut colliders, &motions).join() {
            let bbox = &mut collider.bounding_box;
            bbox.old_position = bbox.position;
            bbox.position.x += motion.velocity.x * time.delta_seconds();
            bbox.position.y += motion.velocity.y * time.delta_seconds();

            let hbox = &mut collider.hit_box;
            hbox.old_position = hbox.position;
            collider.set_hit_box_position(motion.velocity);
        }
    }
}