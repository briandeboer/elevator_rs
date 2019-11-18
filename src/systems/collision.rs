use amethyst::{
    core::Named,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{
        Collider, Motion
    },
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Motion>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, colliders, mut motions, names) = data;

        // this doesn't seem the most efficient way to do this
        for (entity_a, collider_a, motion_a, name_a) in (&entities, &colliders, &mut motions, &names).join() {
            let velocity_a = motion_a.velocity;
            // let bbox_a = &collider_a.bounding_box;
            // let position_a_x = bbox_a.position.x;
            // let half_size_a_x = bbox_a.half_size.x;

            if velocity_a.x != 0. || velocity_a.y != 0. && collider_a.is_collidable {
                for (entity_b, collider_b, name_b) in (&entities, &colliders, &names).join() {
                    if entity_a != entity_b {
                        // let velocity_b = motion_b.velocity;
                        let use_hit_box = false; //(velocity_a.x * velocity_b.x != 0.) || (velocity_a.y * velocity_b.y != 0.);
                        if collider_a.is_overlapping_with(collider_b, use_hit_box) {
                            motion_a.velocity.y = 0.;
                        }
                    }
                }
            }
        }

    }
}