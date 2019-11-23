use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Collidee, Collider, Direction, Elevator, ElevatorComponent, Gun, Motion, Player, PlayerState,
};

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Elevator>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut players, mut elevators, mut colliders, mut collidees, mut motions, mut transforms) =
            data;

        for (maybe_player, maybe_elevator, collider, collidee, motion, transform) in (
            (&mut players).maybe(),
            (&mut elevators).maybe(),
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
            let mut y = bbox.position.y;

            if let Some(collidee_horizontal) = collidee.horizontal.take() {
                bbox.position.x -= collidee_horizontal.correction;
                velocity.x = 0.;
            }
            if let Some(collidee_vertical) = collidee.vertical.take() {
                bbox.position.y -= collidee_vertical.correction;
                velocity.y = 0.;
                if collidee_vertical.correction < 0. {
                    collider.on_ground = true;
                    collider.on_elevator = if collidee_vertical.name == "ElevatorBottom" {
                        true
                    } else {
                        false
                    };
                }
            }
            // FIXME: Due to the take() operation above, collidee.vertical will always be NONE.
            // Might indicate a bug.
            if velocity.y != 0. {
                collider.on_ground = false;
            }
            
            collider.set_hit_box_position(*velocity);
            if let Some(player) = maybe_player {
                if player.state == PlayerState::Ducking {
                    y -= 4.0;
                }
                player.update_position(x, y);
            }

            // FIXME: move this
            // if let Some(elevator) = maybe_elevator {
            //     elevator.update_position(x, y);
            // }

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

pub struct ElevatorTransformationSystem;

impl<'s> System<'s> for ElevatorTransformationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Elevator>,
        ReadStorage<'s, ElevatorComponent>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // let (entities, mut colliders, elevators, components, mut transforms) = data;
        // // loop through all guns and get the parent entity
        // for (_, transform) in
        //     (&components, &mut transforms).join()
        // {
        //     // TODO: clean up duplication of code
        //     // if let Some(top) = maybe_top {
        //     //     for (entity, collider, elevator) in (&entities, &mut colliders, &elevators).join() {
        //     //         if entity == top.parent {
        //     //             // transform.set_translation_y(elevator.pos_y + 24.);
        //     //             let bbox = &mut collider.bounding_box;
        //     //             bbox.position.y = elevator.pos_y;
        //     //             bbox.old_position = bbox.position;
        //     //         }
        //     //     }
        //     // }
        //     // if let Some(bottom) = maybe_bottom {
        //     //     for (entity, collider, elevator) in (&entities, &mut colliders, &elevators).join() {
        //     //         if entity == bottom.parent {
        //     //             // transform.set_translation_y(elevator.pos_y - 24.);
        //     //             let bbox = &mut collider.bounding_box;
        //     //             bbox.position.y = elevator.pos_y;
        //     //             bbox.old_position = bbox.position;
        //     //         }
        //     //     }
        //     // }
        // }
    }
}
