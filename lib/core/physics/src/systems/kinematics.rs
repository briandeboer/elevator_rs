use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider, Motion};

pub struct KinematicsSystem;

impl<'s> System<'s> for KinematicsSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Motion>,
        Read<'s, Time>,
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
