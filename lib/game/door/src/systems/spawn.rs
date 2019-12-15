use amethyst::{
    core::{math::Vector2, timing::Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System},
};

use crate::components::Door;
use asset::{AssetType, PrefabList};
use enemy::{components::Enemy, spawn_enemy};
use floors::Floor;
use rand::Rng;

const MAX_ENEMIES: usize = 3;
const ENEMY_FACTOR: f32 = 0.001;
const TIME_BETWEEN_SPAWNS: f64 = 3.0;

pub struct EnemySpawnSystem;

impl<'s> System<'s> for EnemySpawnSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Door>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Transform>,
        Read<'s, PrefabList>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, doors, enemies, floors, transforms, prefab_list, lazy_update, time) = data;

        // logic for adding an enemy
        // 1. count the number of enemies and check if available
        // 2. look through all the doors that are visible and % chance of spawning
        // 3. spawn an enemy
        // TODO: there's probably a better way to count these
        let mut number_enemies: usize = 0;
        let current_time = time.absolute_time_seconds();
        let mut max_spawn_time: f64 = 0.;
        for (_entity, enemy) in (&entities, &enemies).join() {
            number_enemies += 1;
            max_spawn_time = if enemy.spawn_time > max_spawn_time {
                enemy.spawn_time
            } else {
                max_spawn_time
            };
        }

        let mut rng = rand::thread_rng();
        let mut spawned_from_doors: Vec<u32> = Vec::new();
        if current_time - max_spawn_time < TIME_BETWEEN_SPAWNS {
            return;
        }
        for (entity, door, floor, transform) in (&entities, &doors, &floors, &transforms).join() {
            if number_enemies >= MAX_ENEMIES {
                break;
            }
            // bad guys dont come out of red doors
            if !door.can_user_enter {
                // calculate if we should show an enemy
                let random_number: i32 = rng.gen_range(0, (1. / ENEMY_FACTOR) as i32);
                if random_number == 0 && !spawned_from_doors.contains(&entity.id()) {
                    spawned_from_doors.push(entity.id());
                    number_enemies += 1;
                    let current_translation = transform.translation();
                    let x = current_translation.x;
                    let y = current_translation.y + 12.;
                    spawn_enemy(
                        &entities,
                        &lazy_update,
                        prefab_list.get(AssetType::Enemy).unwrap().clone(),
                        prefab_list.get(AssetType::Guns).unwrap().clone(),
                        Vector2::new(x, y),
                        current_time,
                        floor.floors_overlapped.clone(),
                    );
                }
            }
        }
    }
}
