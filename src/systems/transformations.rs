use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collidee, Collider, Direction, Gun, Motion, Player, PlayerState};

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, mut collidees, mut players, mut motions, mut transforms) = data;

        for (collider, collidee, player, motion, transform) in (
            &mut colliders,
            &mut collidees,
            &mut players,
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
            let x = bbox.position.x;
            let mut y = bbox.position.y;
            if player.state == PlayerState::Ducking {
                y -= 4.0;
            }
            collider.set_hit_box_position(*velocity);
            transform.set_translation_x(x);
            transform.set_translation_y(y);
            player.update_position(x, y);
        }
    }
}

pub struct GunTransformationSystem;

impl<'s> System<'s> for GunTransformationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Gun>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, guns, directions, players, mut transforms) = data;

        // loop through all guns and get the parent entity
        for (gun, direction, transform) in (&guns, &directions, &mut transforms).join() {
            if let Some(parent) = gun.parent {
                for (entity, player) in (&entities, &players).join() {
                    if entity == parent {
                        if direction.x != direction.default_x {
                            transform.set_translation_x(player.pos_x - gun.position_offset.x);
                        } else {
                            transform.set_translation_x(player.pos_x + gun.position_offset.x);
                        }
                        transform.set_translation_y(player.pos_y + gun.position_offset.y);
                    }
                }
            }
        }
    }
}
