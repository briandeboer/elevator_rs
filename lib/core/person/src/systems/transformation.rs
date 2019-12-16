use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Gun, Person, PersonState};
use hierarchy::components::Child;
use physics::components::{Collidee, Collider, Direction, Motion};

const SAFE_PADDING: f32 = 0.1;

pub struct PersonTransformationSystem;

impl<'s> System<'s> for PersonTransformationSystem {
    type SystemData = (
        WriteStorage<'s, Person>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut colliders, mut collidees, mut motions, mut transforms) = data;

        for (player, collider, collidee, motion, transform) in (
            &mut players,
            &mut colliders,
            &mut collidees,
            &mut motions,
            &mut transforms,
        )
            .join()
        {
            // find anything that the player is colliding with
            let bbox = &mut collider.bounding_box;
            let velocity = &mut motion.velocity;

            // if the player collides with something horizontally, he should stop moving
            // and if the thing is rideable it then the player should not be able to push past it
            if let Some(collidee_horizontal) = collidee.horizontal.take() {
                velocity.x = 0.;
                // TODO: change this to mass and velocity?
                if collidee_horizontal.is_rideable {
                    // use the correction to determine which end of the rideable and which way we are getting pushed
                    if collidee_horizontal.correction < 0. {
                        bbox.position.x = collidee_horizontal.position.x
                            + collidee_horizontal.half_size.x
                            + bbox.half_size.x
                            + SAFE_PADDING;
                    } else if collidee_horizontal.correction > 0. {
                        bbox.position.x = collidee_horizontal.position.x
                            - collidee_horizontal.half_size.x
                            - bbox.half_size.x
                            - SAFE_PADDING;
                    }
                } else {
                    bbox.position.x -= collidee_horizontal.correction;
                }
            }
            // if the player collides with something vertically he should stop moving vertically
            // if the player is on somthing rideable he should move with it
            if let Some(collidee_vertical) = collidee.vertical.take() {
                velocity.y = 0.;
                if collidee_vertical.is_rideable {
                    // use the correction to determine which end of the rideable and which way we are getting pushed
                    if collidee_vertical.correction <= 0. {
                        bbox.position.y = collidee_vertical.position.y
                            + collidee_vertical.half_size.y
                            + bbox.half_size.y
                            + SAFE_PADDING;
                    } else {
                        bbox.position.y = collidee_vertical.position.y
                            - collidee_vertical.half_size.y
                            - bbox.half_size.y
                            - SAFE_PADDING;
                    }
                } else {
                    bbox.position.y -= collidee_vertical.correction;
                }

                if collidee_vertical.correction < 0. {
                    collider.on_ground = true;
                }
                if collidee_vertical.name == "ElevatorBottom"
                    || collidee_vertical.name == "ElevatorTop"
                {
                    player.update_ride_velocity(0., collidee_vertical.collided_with_velocity);
                    collider.on_elevator = true;
                }
            } else {
                player.update_ride_velocity(0., 0.);
                collider.on_elevator = false;
            }

            if velocity.y != 0. {
                collider.on_ground = false;
            }

            let x = bbox.position.x;
            let mut y = bbox.position.y;

            collider.set_hit_box_position(*velocity);
            if player.state == PersonState::Ducking {
                y -= 4.0;
            }

            if player.state == PersonState::EnteringRoom {
                transform.set_translation_z(0.);
            } else {
                transform.set_translation_z(0.5);
            }

            player.update_position(x, y);

            transform.set_translation_x(x);
            transform.set_translation_y(y);
        }
    }
}

pub struct GunTransformationSystem;

impl<'s> System<'s> for GunTransformationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Gun>,
        ReadStorage<'s, Child>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Person>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, guns, children, directions, persons, mut transforms) = data;

        // loop through all guns and get the parent entity
        for (_gun, child, direction, transform) in
            (&guns, &children, &directions, &mut transforms).join()
        {
            let parent = child.parent;
            for (entity, person) in (&entities, &persons).join() {
                if entity == parent {
                    if direction.x != direction.default_x {
                        transform.set_translation_x(person.position.x - child.offset_x);
                    } else {
                        transform.set_translation_x(person.position.x + child.offset_x);
                    }
                    transform.set_translation_y(person.position.y + child.offset_y);

                    if person.state == PersonState::EnteringRoom {
                        transform.set_translation_z(0.);
                    } else {
                        transform.set_translation_z(0.7);
                    }
                }
            }
        }
    }
}
