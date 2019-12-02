use amethyst::{
    core::Named,
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Bullet, Collidee, Collider, Gun, Motion, Proximity},
    entities::show_bullet_impact,
    resources::{AssetType, PrefabList},
};

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

const IMPACT_OFFSET_X: f32 = -11.;
const IMPACT_OFFSET_Y: f32 = 25.;

pub struct BulletCollisionSystem;

impl<'s> System<'s> for BulletCollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, Gun>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        ReadExpect<'s, PrefabList>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            bullets,
            mut guns,
            colliders,
            collidees,
            mut motions,
            prefab_list,
            lazy_update,
        ) = data;

        for (entity, bullet, collider, collidee, motion) in
            (&*entities, &bullets, &colliders, &collidees, &mut motions).join()
        {
            // Currently, bullet can be fired only horizontally
            if let Some(collidee_horizontal) = &collidee.horizontal {
                match collidee_horizontal.name {
                    // "Boundary" => {}
                    _ => {
                        let bullet_impact_prefab_handle =
                            { prefab_list.get(AssetType::BulletImpact).unwrap().clone() };
                        let impact_position_x = if motion.velocity.x > 0. {
                            collidee_horizontal.position.x + IMPACT_OFFSET_X
                        } else {
                            collidee_horizontal.position.x + IMPACT_OFFSET_Y
                        };
                        show_bullet_impact(
                            &entities,
                            bullet_impact_prefab_handle,
                            impact_position_x,
                            collider.bounding_box.position.y,
                            motion.velocity.x,
                            &lazy_update,
                        );
                    }
                }
                // get the gun to remove a shot
                for (entity, gun) in (&entities, &mut guns).join() {
                    if let Some(parent) = bullet.parent {
                        if parent == entity {
                            gun.shots_fired -= 1;
                        }
                    }
                }
                let _ = entities.delete(entity);
            }
        }
    }
}
