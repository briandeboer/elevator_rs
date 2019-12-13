use amethyst::{
    assets::{Asset, Handle, Prefab, ProcessingState},
    core::{math::Vector2, transform::Transform, Named, WithNamed},
    ecs::{Builder, Entities, Entity, LazyUpdate, ReadExpect, VecStorage, World, WorldExt},
    error::Error,
    renderer::{sprite::SpriteSheetHandle, SpriteRender, SpriteSheet},
};

use serde::{Deserialize, Serialize};

use animation::components::AnimationPrefabData;
use door::load_door;
use elevator::load_elevator;
use floors::{Floor, FloorsDrawn};
use physics::components::{Collider, Direction, Motion};

const OFFSET_X: f32 = 0.0;
const OFFSET_Y: f32 = 224.0;
const TILE_WIDTH: f32 = 256.0;
const TILE_HEIGHT: f32 = 48.0;
const NUM_COLUMNS: usize = 1;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Property {
    pub name: String,
    pub value: usize,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub id: usize,
    pub name: String,
    pub height: f32,
    pub width: f32,
    pub rotation: f32,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub properties: Option<Vec<Property>>,
    pub floors_overlapped: Option<Vec<usize>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Layer {
    pub name: String,
    pub data: Option<Vec<i32>>,
    pub height: Option<i32>,
    pub opacity: i32,
    pub width: Option<i32>,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub objects: Option<Vec<Object>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub layers: Vec<Layer>,
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map) -> Result<ProcessingState<Map>, Error> {
        Ok(ProcessingState::Loaded(map))
    }
}

impl Asset for Map {
    const NAME: &'static str = "elevator_rs::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Map>>;
}

impl Map {
    pub fn init_floors(&mut self, world: &mut World) {
        // set a new floors drawn resource
        let mut floors = FloorsDrawn::default();
        for layer in self.layers.iter() {
            if layer.name == "floors" {
                self.load_floor_boundaries(layer, &mut floors);
            }
        }
        world.insert(floors);
        self.add_floors_to_layers(world);
    }

    pub fn add_floors_to_layers(&mut self, world: &mut World) {
        for layer in self.layers.iter_mut() {
            match layer.name.as_ref() {
                "floors" => {
                    // ignored
                }
                _ => {
                    // loop through objects and add their list of floors
                    if let Some(objects) = &mut layer.objects {
                        for obj in objects.iter_mut() {
                            let x = OFFSET_X + obj.x + (obj.width / 2.);
                            let y = OFFSET_Y - obj.y - (obj.height / 2.);
                            let position = Vector2::new(x, y);
                            let floors_overlapped: Vec<usize> = {
                                let floors = world.read_resource::<FloorsDrawn>();
                                // do some work to find the floor number
                                floors.find_floors(position, obj.width, obj.height)
                            };
                            obj.floors_overlapped = match floors_overlapped.len() {
                                0 => None,
                                _ => Some(floors_overlapped),
                            };
                        }
                    }
                }
            }
        }
    }

    pub fn get_layer(&self, layer_name: &str) -> Option<&Layer> {
        self.layers.iter().find(|l| l.name == layer_name)
    }

    fn load_floor_boundaries(&self, layer: &Layer, floors: &mut FloorsDrawn) {
        if let Some(objects) = &layer.objects {
            for (_index, obj) in objects.iter().enumerate() {
                let mut floor_number: usize = 0;
                let x = OFFSET_X + obj.x + (obj.width / 2.);
                let y = OFFSET_Y - obj.y - (obj.height / 2.);
                if let Some(properties) = &obj.properties {
                    for property in properties {
                        if property.name == "floor" {
                            floor_number = property.value as usize;
                        }
                    }
                }
                println!(
                    "Adding floor boundaries: floor: {}, x: {}, y: {}, width: {}, height: {}",
                    floor_number, obj.x, obj.y, obj.width, obj.height
                );
                floors.add_boundary(floor_number, Vector2::new(x, y), obj.width, obj.height);
            }
        }
    }

    pub fn render_collisions(
        &self,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
        floors_to_draw: &Vec<usize>,
        rendered_ids: &mut Vec<usize>,
    ) {
        let scale_x: f32 = 1.0;
        let scale_y: f32 = 1.0;

        if let Some(layer) = self.get_layer("collision") {
            if let Some(objects) = &layer.objects {
                for (_index, obj) in objects.iter().enumerate() {
                    if let Some(floors_overlapped) = &obj.floors_overlapped {
                        if !rendered_ids.contains(&obj.id)
                            && should_draw(floors_overlapped, floors_to_draw)
                        {
                            let collision_entity: Entity = entities.create();
                            let mut transform = Transform::default();
                            let mut collider =
                                Collider::new(obj.width * scale_x, obj.height * scale_y);
                            let bbox = &mut collider.bounding_box;
                            if obj.name == "shaft" || obj.name == "entry" {
                                collider.is_collidable = false;
                            }
                            let x = OFFSET_X + obj.x;
                            let y = OFFSET_Y - obj.y;
                            transform.set_translation_z(-10.0);
                            if let Some(floors_overlapped) = &obj.floors_overlapped {
                                println!(
                                    "### Adding collision object {}, x: {}, y: {}, width: {}, height: {}, floors: {:?} ###",
                                    obj.name, x, y, obj.width, obj.height, floors_overlapped
                                );
                            }
                            bbox.position = Vector2::new(
                                OFFSET_X + (obj.x * scale_x) + bbox.half_size.x,
                                OFFSET_Y - (obj.y * scale_y) - bbox.half_size.y,
                            );
                            bbox.old_position = bbox.position;
                            lazy_update
                                .insert(collision_entity, Named::new(String::from(&obj.name)));
                            lazy_update.insert(collision_entity, Motion::new());
                            lazy_update.insert(collision_entity, transform);
                            lazy_update.insert(collision_entity, collider);
                            lazy_update.insert(collision_entity, Direction::default());
                            lazy_update.insert(
                                collision_entity,
                                Floor::new(vec![obj.id], floors_overlapped.clone()),
                            );
                            rendered_ids.push(obj.id);
                        }
                    }
                }
            }
        }
    }

    pub fn render_doors(
        &self,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
        prefab_handle: Handle<Prefab<AnimationPrefabData>>,
        floors_to_draw: &Vec<usize>,
        rendered_ids: &mut Vec<usize>,
    ) {
        if let Some(layer) = self.get_layer("doors") {
            if let Some(objects) = &layer.objects {
                for (_index, obj) in objects.iter().enumerate() {
                    if let Some(floors_overlapped) = &obj.floors_overlapped {
                        if !rendered_ids.contains(&obj.id)
                            && should_draw(floors_overlapped, floors_to_draw)
                        {
                            let x = OFFSET_X + obj.x + (obj.width / 2.);
                            // FIXME: need to figure out why doors are off by 1 pixel
                            let y = OFFSET_Y - obj.y - (obj.height / 2.) - 1.;
                            println!(
                                "### Adding door object {}, x: {}, y: {}, width: {}, height: {} ###",
                                obj.name, x, y, obj.width, obj.height
                            );
                            rendered_ids.push(obj.id);
                            load_door(
                                obj.id,
                                entities,
                                lazy_update,
                                prefab_handle.clone(),
                                Vector2::new(x, y),
                                &obj.name,
                                &floors_overlapped,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn render_elevators(
        &self,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
        sprite_sheet_handle: SpriteSheetHandle,
        floors_to_draw: &Vec<usize>,
        rendered_ids: &mut Vec<usize>,
    ) {
        if let Some(layer) = self.get_layer("elevators") {
            if let Some(objects) = &layer.objects {
                for (_index, obj) in objects.iter().enumerate() {
                    if let Some(floors_overlapped) = &obj.floors_overlapped {
                        if !rendered_ids.contains(&obj.id)
                            && should_draw(floors_overlapped, floors_to_draw)
                        {
                            let x = OFFSET_X + obj.x + (obj.width / 2.);
                            let y = OFFSET_Y - obj.y - (48. / 2.);
                            let mut min_floor: usize = 0;
                            let mut max_floor: usize = 0;
                            let mut start_floor: usize = 0;
                            if let Some(properties) = &obj.properties {
                                for property in properties {
                                    if property.name == "min_floor" {
                                        min_floor = property.value;
                                    } else if property.name == "max_floor" {
                                        max_floor = property.value;
                                    } else if property.name == "start_floor" {
                                        start_floor = property.value;
                                    }
                                }
                            }
                            println!(
                                "### Adding elevator object {:?}, x: {}, y: {}, min: {}, max: {}, start: {} ###",
                                obj, x, y, min_floor, max_floor, start_floor,
                            );
                            let top_left = Vector2::new(x, y);
                            let bottom_right = Vector2::new(x + 48., y - obj.height);

                            rendered_ids.push(obj.id);
                            // TODO: add the ids
                            load_elevator(
                                obj.id,
                                entities,
                                lazy_update,
                                sprite_sheet_handle.clone(),
                                top_left,
                                bottom_right,
                                min_floor,
                                max_floor,
                                start_floor,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn render_tiles(&self, world: &mut World, sprite_sheet_handle: &Handle<SpriteSheet>) {
        // TODO: support drawing in different directions
        // TODO: support different tileset spacing and margins
        // get the ssprite sheet handle
        if let Some(layer) = self.get_layer("map") {
            if let Some(data) = &layer.data {
                for (index, d) in data.iter().enumerate() {
                    let x = TILE_WIDTH / 2.0 + TILE_HEIGHT * (index % NUM_COLUMNS) as f32; // offset is half tile width
                    let y =
                        OFFSET_Y - TILE_HEIGHT / 2.0 - (TILE_HEIGHT * (index / NUM_COLUMNS) as f32);

                    let tile_sprite = SpriteRender {
                        sprite_sheet: sprite_sheet_handle.clone(),
                        sprite_number: (*d - 1) as usize, // sprite numbers are off by 1 in t
                    };
                    let mut tile_transform = Transform::default();
                    // TODO: make these not hardcoded
                    tile_transform.set_translation_xyz(x, y, -10.0);
                    world
                        .create_entity()
                        .named("map_tile")
                        .with(tile_transform)
                        .with(tile_sprite)
                        .build();
                }
            }
        }
    }
}

fn should_draw(floors_overlapped: &Vec<usize>, floors_to_draw: &Vec<usize>) -> bool {
    for f in floors_overlapped {
        if floors_to_draw.contains(&f) {
            return true;
        }
    }
    false
}

// fn load_sprite_layer(world: &mut World, layer: &Layer, floors_to_draw: &Vec<usize>) {
//     if let Some(objects) = &layer.objects {
//         for (_index, obj) in objects.iter().enumerate() {
//             if let Some(floors_overlapped) = &obj.floors_overlapped {
//                 if should_draw(floors_overlapped, floors_to_draw) {
//                     if layer.name == "doors" {
//                         let x = OFFSET_X + obj.x + (obj.width / 2.);
//                         // FIXME: need to figure out why doors are off by 1 pixel
//                         let y = OFFSET_Y - obj.y - (obj.height / 2.) - 1.;
//                         println!(
//                             "### Adding door object {}, x: {}, y: {}, width: {}, height: {} ###",
//                             obj.name, x, y, obj.width, obj.height
//                         );
//                         let prefab_handle = {
//                             let prefab_list = world.read_resource::<PrefabList>();
//                             prefab_list.get(AssetType::Door).unwrap().clone()
//                         };
//                         load_door(world, prefab_handle, Vector2::new(x, y), &obj.name);
//                     } else if layer.name == "elevators" {
//                         let x = OFFSET_X + obj.x + (obj.width / 2.);
//                         let y = OFFSET_Y - obj.y - (48. / 2.);
//                         let mut min_floor: usize = 0;
//                         let mut max_floor: usize = 0;
//                         let mut start_floor: usize = 0;
//                         if let Some(properties) = &obj.properties {
//                             for property in properties {
//                                 if property.name == "min_floor" {
//                                     min_floor = property.value;
//                                 } else if property.name == "max_floor" {
//                                     max_floor = property.value;
//                                 } else if property.name == "start_floor" {
//                                     start_floor = property.value;
//                                 }
//                             }
//                         }
//                         println!(
//                             "### Adding elevator object {:?}, x: {}, y: {}, min: {}, max: {}, start: {} ###",
//                             obj, x, y, min_floor, max_floor, start_floor,
//                         );
//                         let elevator_sprite_sheet_handle = {
//                             let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
//                             sprite_sheet_list.get(AssetType::Elevator).unwrap().clone()
//                         };
//                         let top_left = Vector2::new(x, y);
//                         let bottom_right = Vector2::new(x + 48., y - obj.height);

//                         load_elevator(
//                             world,
//                             elevator_sprite_sheet_handle,
//                             top_left,
//                             bottom_right,
//                             min_floor,
//                             max_floor,
//                             start_floor,
//                         );
//                     }
//                 }
//             }
//         }
//     }
// }
