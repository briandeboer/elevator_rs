use amethyst::{
    core::Named,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collidee, Collider, Motion};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, colliders, mut collidees, motions, names) = data;

        // this doesn't seem the most efficient way to do this
        for (entity_a, collider_a, collidee, motion_a, _name_a) in
            (&entities, &colliders, &mut collidees, &motions, &names).join()
        {
            let velocity_a = motion_a.velocity;
            let bbox_a = &collider_a.bounding_box;
            let _position_a_x = bbox_a.position.x;
            let _half_size_a_x = bbox_a.half_size.x;

            if velocity_a.x != 0. || velocity_a.y != 0. && collider_a.is_collidable {
                for (entity_b, collider_b, motion_b, name_b) in
                    (&entities, &colliders, &motions, &names).join()
                {
                    if entity_a != entity_b {
                        let velocity_b = motion_b.velocity;
                        let use_hit_box = (velocity_a.x * velocity_b.x != 0.)
                            || (velocity_a.y * velocity_b.y != 0.);
                        if collider_a.is_overlapping_with(collider_b, use_hit_box) {
                            collidee.set_collidee_details(
                                name_b.name.to_string(),
                                collider_a,
                                collider_b,
                                velocity_a,
                                velocity_b,
                                use_hit_box,
                            );
                        }
                    }
                }
            }
        }
    }
}
