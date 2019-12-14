use amethyst::core::timing::Time;
use amethyst::core::{Named, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, SystemData, World, Write,
};

const REMOVE_WAIT_TIME: f64 = 1.0;
const MAX_FLOOR: usize = 31;
const MIN_FLOOR: usize = 0;
const FLOORS_TO_RENDER: usize = 5;

use crate::Map;
use array_tool::vec::Intersect;
use asset::{AssetType, PrefabList, SpriteSheetList};
use floors::{Floor, FloorsDrawn};
use person::components::Person;
use player::components::Player;

#[derive(SystemDesc)]
pub struct MapRenderSystem;

impl<'s> System<'s> for MapRenderSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Person>,
        ReadStorage<'s, Player>,
        Read<'s, Map>,
        Write<'s, FloorsDrawn>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Named>,
        Read<'s, Time>,
        Read<'s, PrefabList>,
        Read<'s, SpriteSheetList>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            persons,
            players,
            map,
            mut floors_drawn,
            floors,
            names,
            time,
            prefab_list,
            sprite_sheet_list,
            lazy_update,
        ) = data;

        // determine which floor the player is on
        let mut current_floors: Vec<usize> = Vec::new();
        for (_entity, person, _player) in (&entities, &persons, &players).join() {
            // TODO: actually calculate here
            current_floors = floors_drawn.find_floors(person.position, 12., 24.);
        }

        // render any floors that are not drawn
        let mut floors_to_draw: Vec<usize> = current_floors.clone();
        while current_floors.len() > 0 && floors_to_draw.len() < FLOORS_TO_RENDER {
            // start at max and min add those, and add +1 and -1
            let max = floors_to_draw.iter().cloned().max().unwrap();
            let min = floors_to_draw.iter().cloned().min().unwrap();
            if max < MAX_FLOOR {
                floors_to_draw.push(max + 1);
            }
            if min > MIN_FLOOR && floors_to_draw.len() < FLOORS_TO_RENDER {
                floors_to_draw.push(min - 1);
            }
        }
        let pending_draw: Vec<usize> = floors_to_draw
            .iter()
            .filter(|f| !floors_drawn.rendered_floors.contains(&f))
            .copied()
            .collect();

        // check for floors that are drawn that shouldn't be drawn
        // and that are already not pending
        let current_time: f64 = time.absolute_time_seconds();
        // look through current pending floors for anything in floors to draw
        let mut pending_removal: Vec<(usize, f64)> = floors_drawn
            .pending_removal
            .iter()
            .filter(|(f, _)| !floors_to_draw.contains(&f))
            .copied()
            .collect();
        // add floors that aren't current anymore
        for f in &floors_drawn.rendered_floors {
            if !floors_to_draw.contains(&f)
                && pending_removal.iter().cloned().find(|(x, _)| x == f) == None
            {
                // check if they are already in pending state
                pending_removal.push((f.clone(), current_time));
            }
        }

        // render stuff
        let mut rendered_ids = floors_drawn.rendered_ids.clone();
        map.render_collisions(&entities, &lazy_update, &pending_draw, &mut rendered_ids);
        map.render_doors(
            &entities,
            &lazy_update,
            prefab_list.get(AssetType::Door).unwrap().clone(),
            &pending_draw,
            &mut rendered_ids,
        );
        // TODO: NEED TO CHECK FIRST IF THE ELEVATOR IS ALREADY DRAWN
        // probably need to give every object an id and keep track of it that way
        // what about objects that are dynamically drawn, like enemies...? (use entityid)
        map.render_elevators(
            &entities,
            &lazy_update,
            sprite_sheet_list.get(AssetType::Elevator).unwrap().clone(),
            &pending_draw,
            &mut rendered_ids,
        );

        // set the rendered floors to have the new ones
        let mut all_drawn = [floors_drawn.rendered_floors.clone(), floors_to_draw.clone()].concat();
        all_drawn.sort();
        all_drawn.dedup();

        // remove stuff
        let mut still_pending: Vec<(usize, f64)> = Vec::new();
        for (floor_to_remove, time) in &pending_removal {
            if time < &(current_time - REMOVE_WAIT_TIME) {
                // remove this floor
                for (entity, floor, named) in (&entities, &floors, &names).join() {
                    if floor.contains(floor_to_remove) {
                        // check for intersection
                        let intersection: Vec<usize> =
                            floors_to_draw.intersect(floor.floors_overlapped.clone());
                        if intersection.len() == 0 {
                            println!(
                                "### Removing entity: {}, floors: {:?}",
                                named.name, floor.floors_overlapped
                            );
                            println!(
                                "Floors to draw: {:?}, floor to remove: {}",
                                floors_to_draw, floor_to_remove
                            );
                            // remove the ids
                            rendered_ids = rendered_ids
                                .iter()
                                .filter(|id| !floor.object_ids.contains(id))
                                .copied()
                                .collect();
                            let _ = entities.delete(entity);
                        }
                    }
                }
            } else {
                still_pending.push((floor_to_remove.clone(), time.clone()));
            }
        }

        floors_drawn.rendered_ids = rendered_ids;
        floors_drawn.pending_removal = still_pending;
        floors_drawn.rendered_floors = floors_to_draw;
    }
}
