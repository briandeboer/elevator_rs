use amethyst::ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::{Bullet, Gun, Person, PersonState};
use physics::components::{Collidee, Collider, Motion};

use crate::bullet::show_bullet_impact;
use asset::{AssetType, PrefabList};

const IMPACT_OFFSET_X: f32 = -8.;

pub struct BulletCollisionSystem;

impl<'s> System<'s> for BulletCollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Person>,
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
            mut persons,
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
                let bullet_impact_prefab_handle =
                    { prefab_list.get(AssetType::BulletImpact).unwrap().clone() };
                let impact_position_x = if motion.velocity.x > 0. {
                    collidee_horizontal.position.x + IMPACT_OFFSET_X
                } else {
                    collidee_horizontal.position.x - IMPACT_OFFSET_X
                };
                if collidee_horizontal.name == "Enemy" || collidee_horizontal.name == "Player" {
                    // they should die
                    for (person_entity, person) in (&entities, &mut persons).join() {
                        if person_entity == collidee_horizontal.collided_with_entity {
                            person.state = PersonState::Dying;
                            break;
                        }
                    }
                }
                println!("collidee_horizontal.name: {}", collidee_horizontal.name);
                show_bullet_impact(
                    &entities,
                    bullet_impact_prefab_handle,
                    impact_position_x,
                    collider.bounding_box.position.y,
                    motion.velocity.x,
                    &lazy_update,
                );
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
