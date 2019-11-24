use amethyst::{
    core::Named,
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Bullet, Collidee, Collider, Gun, Motion},
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

        // this doesn't seem the most efficient way to do this
        for (entity_a, collider_a, collidee, motion_a, name_a) in
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
                    if entity_a != entity_b && collider_b.is_collidable {
                        let velocity_b = motion_b.velocity;
                        let use_hit_box = (velocity_a.x * velocity_b.x != 0.)
                            || (velocity_a.y * velocity_b.y != 0.);
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
                        }
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

        for (entity, bullet, collider, collidee, motion) in (
            &*entities,
            &bullets,
            &colliders,
            &collidees,
            &mut motions,
        )
            .join()
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
