use amethyst::{
    core::{math::Vector2, timing::Time},
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Enemy;

use array_tool::vec::Intersect;
use floors::Floor;
use person::components::{Person, PersonState};
use physics::components::{Direction, Directions};
use player::components::Player;
use rand::Rng;

const PURSUIT_FACTOR_SAME_FLOOR: f32 = 0.05;
// const CONTINUE_PURSUIT_FACTOR: f32 = 0.1;
// const PURSUIT_DIFFERENT_FLOOR: f32 = 0.001;
const MIN_PURSUIT_TIME: f64 = 1.0;

pub struct EnemyAISystem;

impl<'s> System<'s> for EnemyAISystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Person>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Floor>,
        WriteStorage<'s, Direction>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut enemies, mut persons, players, floors, mut directions, time) = data;
        let current_time = time.absolute_time_seconds();
        let mut rng = rand::thread_rng();

        // start with this...
        // a) if the player is on the same floor, turn towards them
        // b) fire a gun when possible

        // 1. get the player floor and location
        let mut player_position: Vector2<f32> = Vector2::new(0., 0.);
        let mut player_floor: Vec<usize> = Vec::new();
        let mut player_entity: Option<Entity> = None;
        for (entity, person, _player, floor) in (&entities, &persons, &players, &floors).join() {
            player_position.x = person.position.x;
            player_position.y = person.position.y;
            player_floor = floor.floors_overlapped.clone();
            player_entity = Some(entity);
        }

        // 2. find all plauyers
        if let Some(pursuit_entity) = player_entity {
            for (_entity, person, enemy, floor, direction) in (
                &entities,
                &mut persons,
                &mut enemies,
                &floors,
                &mut directions,
            )
                .join()
            {
                let overlap = player_floor.intersect(floor.floors_overlapped.clone());
                if overlap.len() > 0 {
                    if person.position.x >= player_position.x {
                        direction.x = Directions::Left;
                    } else {
                        direction.x = Directions::Right;
                    }
                    // determine if pursuing
                    if enemy.pursuing_entity.is_some() {
                        if (current_time - enemy.pursuit_time) > MIN_PURSUIT_TIME {
                            let random_number: i32 = rng.gen_range(0, 2);
                            if random_number == 0 {
                                enemy.pursuing_entity = Some(pursuit_entity);
                            } else {
                                enemy.pursuing_entity = None;
                            }
                        }
                    } else {
                        let random_number: i32 =
                            rng.gen_range(0, (1. / PURSUIT_FACTOR_SAME_FLOOR) as i32);
                        if random_number == 0 {
                            enemy.pursuit_time = current_time;
                            enemy.pursuing_entity = Some(pursuit_entity);
                        } else {
                            enemy.pursuing_entity = None;
                        }
                    }

                    if person.state != PersonState::Dying {
                        if enemy.pursuing_entity.is_some() {
                            person.state = PersonState::Walking;
                        } else {
                            person.state = PersonState::Idling;
                        }
                    }
                }
            }
        }

        // TODO:
        // if on the same floor, pursue automatically
        //      % chance for shooting at user based on time since last shot
        // on a different floor
        //      decide if enemy is pursuing
        //
    }
}
