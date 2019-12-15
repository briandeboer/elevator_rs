use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
use std::collections::HashMap;

use crate::components::Person;
use floors::{Floor, FloorsDrawn};
use hierarchy::components::Child;

pub struct PersonFloorSystem;

impl<'s> System<'s> for PersonFloorSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, FloorsDrawn>,
        ReadStorage<'s, Child>,
        ReadStorage<'s, Person>,
        WriteStorage<'s, Floor>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, floors_drawn, children, persons, mut floors) = data;

        // create a hash map to know floors to entity map
        let mut floor_map: HashMap<u32, Vec<usize>> = HashMap::new();

        for (entity, person, floor) in (&entities, &persons, &mut floors).join() {
            // determine the floors overlapped and set accordingly
            // determine which floor the person is on
            floor.floors_overlapped = floors_drawn.find_floors(person.position, 12., 24.).clone();
            floor_map.insert(entity.id(), floor.floors_overlapped.clone());
        }

        for (_entity, child, floor) in (&entities, &children, &mut floors).join() {
            if floor_map.contains_key(&child.parent.id()) {
                floor.floors_overlapped = floor_map.get(&child.parent.id()).unwrap().to_vec();
            }
        }
    }
}
