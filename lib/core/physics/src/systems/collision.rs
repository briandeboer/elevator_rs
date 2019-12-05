use amethyst::{
    core::Named,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collidee, Collider, Motion, Proximity};

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

        for (entity_a, collider_a, collidee, motion_a, name_a) in
            (&entities, &colliders, &mut collidees, &motions, &names).join()
        {
            let velocity_a = motion_a.velocity;
            if velocity_a.x != 0. || velocity_a.y != 0. && collider_a.is_collidable {
                for (entity_b, collider_b, motion_b, name_b) in
                    (&entities, &colliders, &motions, &names).join()
                {
                    let velocity_b = motion_b.velocity;
                    let use_hit_box =
                        (velocity_a.x * velocity_b.x != 0.) || (velocity_a.y * velocity_b.y != 0.);
                    if entity_a != entity_b && collider_b.is_collidable {
                        if collider_a.is_overlapping_with(collider_b, use_hit_box) {
                            collidee.set_collidee_details(
                                name_b.name.to_string(),
                                name_a.name.to_string(),
                                collider_a,
                                collider_b,
                                velocity_a,
                                velocity_b,
                                use_hit_box,
                            );
                            // println!("name_a: {}, name_b: {}", name_a.name.to_string(), name_b.name.to_string());
                        }
                    }
                }
            }
        }
    }
}

pub struct ProximitySystem;

impl<'s> System<'s> for ProximitySystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Named>,
        WriteStorage<'s, Proximity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, colliders, motions, names, mut proximities) = data;

        for (entity_a, collider_a, motion_a, name_a, proximity_a) in
            (&entities, &colliders, &motions, &names, &mut proximities).join()
        {
            let velocity_a = motion_a.velocity;
            if velocity_a.x != 0. || velocity_a.y != 0. && collider_a.allow_proximity {
                proximity_a.reset_details();
                for (entity_b, collider_b, motion_b, name_b) in
                    (&entities, &colliders, &motions, &names).join()
                {
                    let velocity_b = motion_b.velocity;
                    let use_hit_box =
                        (velocity_a.x * velocity_b.x != 0.) || (velocity_a.y * velocity_b.y != 0.);
                    if entity_a != entity_b && collider_b.allow_proximity {
                        proximity_a.add_proximity_details(
                            name_a.name.to_string(),
                            name_b.name.to_string(),
                            collider_a,
                            collider_b,
                            velocity_a,
                            use_hit_box,
                        );
                    }
                }
            }
        }
    }
}
