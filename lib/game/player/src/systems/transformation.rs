use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::Camera,
};

use crate::components::{Gun, Player, PlayerState};
use hierarchy::components::Child;
use physics::components::{Collidee, Collider, Direction, Motion};

const SAFE_PADDING: f32 = 0.01;

pub struct PlayerTransformationSystem;

impl<'s> System<'s> for PlayerTransformationSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
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
            if player.state == PlayerState::Ducking {
                y -= 4.0;
            }

            if player.state == PlayerState::EnteringRoom {
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
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, guns, children, directions, players, mut transforms) = data;

        // loop through all guns and get the parent entity
        for (_gun, child, direction, transform) in
            (&guns, &children, &directions, &mut transforms).join()
        {
            let parent = child.parent;
            for (entity, player) in (&entities, &players).join() {
                if entity == parent {
                    if direction.x != direction.default_x {
                        transform.set_translation_x(player.position.x - child.offset_x);
                    } else {
                        transform.set_translation_x(player.position.x + child.offset_x);
                    }
                    transform.set_translation_y(player.position.y + child.offset_y);

                    if player.state == PlayerState::EnteringRoom {
                        transform.set_translation_z(0.);
                    } else {
                        transform.set_translation_z(0.7);
                    }
                }
            }
        }
    }
}

pub struct CameraTransformationSystem;

const CAMERA_MOVE_FACTOR: f32 = 35.;

impl<'s> System<'s> for CameraTransformationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, players, cameras, mut transforms) = data;
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            for (_, player) in (&entities, &players).join() {
                if player.state != PlayerState::Jumping && player.state != PlayerState::Ducking {
                    // move towards it but not too fast
                    let current_translation = transform.translation();
                    let new_y = current_translation.y
                        + (player.position.y - current_translation.y) / CAMERA_MOVE_FACTOR;
                    transform.set_translation_y(new_y);
                }
                // for now we only support one player at a time
                break;
            }
        }
    }
}
