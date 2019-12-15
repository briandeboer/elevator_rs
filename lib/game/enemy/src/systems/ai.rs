use amethyst::{
    core::{math::Vector2, timing::Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System},
};

use crate::components::Door;
use asset::{AssetType, PrefabList};
use enemy::{components::Enemy, spawn_enemy};
use floors::Floor;
use rand::Rng;

pub struct EnemyAISystem;

impl<'s> System<'s> for EnemyAISystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, enemies, floors, transforms, lazy_update, time) = data;

        // start with this...
        // if the player is on the same floor, turn towards them
        // fire a gun when possible
        

        // TODO:
        // if on the same floor, pursue automatically
        //      % chance for shooting at user based on time since last shot
        // on a different floor
        //      decide if enemy is pursuing
        //      

    }
}
