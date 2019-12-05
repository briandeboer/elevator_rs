use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collidee, Collider, DefaultTransformation, Motion};

pub struct DefaultTransformationSystem;

impl<'s> System<'s> for DefaultTransformationSystem {
    type SystemData = (
        ReadStorage<'s, DefaultTransformation>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (default_transformations, mut colliders, mut collidees, mut motions, mut transforms) =
            data;

        for (_, collider, collidee, motion, transform) in (
            &default_transformations,
            &mut colliders,
            &mut collidees,
            &mut motions,
            &mut transforms,
        )
            .join()
        {
            let bbox = &mut collider.bounding_box;
            let velocity = &mut motion.velocity;

            let x = bbox.position.x;
            let y = bbox.position.y;

            if let Some(collidee_horizontal) = collidee.horizontal.take() {
                bbox.position.x -= collidee_horizontal.correction;
                velocity.x = 0.;
            }
            if let Some(collidee_vertical) = collidee.vertical.take() {
                velocity.y = 0.;
                if collidee_vertical.correction < 0. {
                    collider.on_ground = true;
                }
            }

            if velocity.y != 0. {
                collider.on_ground = false;
            }

            collider.set_hit_box_position(*velocity);

            transform.set_translation_x(x);
            transform.set_translation_y(y);
        }
    }
}
