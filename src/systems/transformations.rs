use amethyst::{
    core::Transform,
    ecs::{Join, System, WriteStorage},
};

use crate::{
    components::{Collidee, Collider, Motion},
};

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, mut collidees, mut motions, mut transforms) = data;

        for (collider, collidee, motion, transform) in (
            &mut colliders,
            &mut collidees,
            &mut motions,
            &mut transforms,
        )
            .join()
        {
            let bbox = &mut collider.bounding_box;
            let velocity = &mut motion.velocity;

            if let Some(collidee_horizontal) = collidee.horizontal.take() {
                bbox.position.x -= collidee_horizontal.correction;
                velocity.x = 0.;
            }
            if let Some(collidee_vertical) = collidee.vertical.take() {
                bbox.position.y -= collidee_vertical.correction;
                velocity.y = 0.;
                if collidee_vertical.correction < 0. {
                    collider.on_ground = true;
                }
            }
            // FIXME: Due to the take() operation above, collidee.vertical will always be NONE.
            // Might indicate a bug.
            if velocity.y != 0. {
                collider.on_ground = false;
            }
            transform.set_translation_x(bbox.position.x);
            transform.set_translation_y(bbox.position.y);
            collider.set_hit_box_position(*velocity);
        }
    }
}
