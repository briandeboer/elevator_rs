use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    core::{math::Vector2, transform::Transform, WithNamed},
    ecs::{Builder, VecStorage, World, WorldExt},
    error::Error,
    renderer::{SpriteRender, SpriteSheet},
};

use serde::{Deserialize, Serialize};

use asset::{AssetType, PrefabList, SpriteSheetList};
use door::load_door;
use elevator::load_elevator;
use physics::components::{Collider, Direction, Motion};

const TILE_OFFSET_Y: f32 = 224.0;
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
    pub name: String,
    pub height: f32,
    pub width: f32,
    pub rotation: f32,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub properties: Option<Vec<Property>>,
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
    pub fn load_layers(&self, world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
        println!("### Load layers ###");

        for layer in self.layers.iter() {
            match layer.name.as_ref() {
                "collision" => {
                    self.load_collision_layer(world, layer);
                }
                "doors" => {
                    self.load_sprite_layer(world, layer);
                }
                "map" => {
                    self.load_tile_layer(world, layer, &sprite_sheet_handle);
                }
                "elevators" => {
                    self.load_sprite_layer(world, layer);
                }
                _ => {
                    // do nothing with the other layers...yet
                }
            }
        }
    }

    fn load_sprite_layer(&self, world: &mut World, layer: &Layer) {
        // let scale_x: f32 = 1.0;
        // let scale_y: f32 = 1.0;
        let offset_x: f32 = 0.0;
        let offset_y: f32 = 223.;
        if let Some(objects) = &layer.objects {
            for (_index, obj) in objects.iter().enumerate() {
                if layer.name == "doors" {
                    let x = offset_x + obj.x + (obj.width / 2.);
                    let y = offset_y - obj.y - (obj.height / 2.);
                    println!(
                        "### Adding door object {}, x: {}, y: {}, width: {}, height: {} ###",
                        obj.name, x, y, obj.width, obj.height
                    );
                    let prefab_handle = {
                        let prefab_list = world.read_resource::<PrefabList>();
                        prefab_list.get(AssetType::Door).unwrap().clone()
                    };
                    load_door(world, prefab_handle, Vector2::new(x, y), &obj.name);
                } else if layer.name == "elevators" {
                    let x = offset_x + obj.x + (obj.width / 2.);
                    let y = offset_y - obj.y - (48. / 2.);
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
                    let elevator_sprite_sheet_handle = {
                        let sprite_sheet_list = world.read_resource::<SpriteSheetList>();
                        sprite_sheet_list.get(AssetType::Elevator).unwrap().clone()
                    };
                    // FIXME: elevator y needs to be offset by 1...not sure why yet?
                    let top_left = Vector2::new(x, y + 1.);
                    let bottom_right = Vector2::new(x + 48., y - obj.height);

                    load_elevator(
                        world,
                        elevator_sprite_sheet_handle,
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

    fn load_collision_layer(&self, world: &mut World, layer: &Layer) {
        let scale_x: f32 = 1.0;
        let scale_y: f32 = 1.0;
        let offset_x: f32 = 0.0;
        let offset_y: f32 = 223.;

        if let Some(objects) = &layer.objects {
            for (_index, obj) in objects.iter().enumerate() {
                let mut transform = Transform::default();
                let mut collider = Collider::new(obj.width * scale_x, obj.height * scale_y);
                let bbox = &mut collider.bounding_box;
                if obj.name == "shaft" || obj.name == "entry" {
                    collider.is_collidable = false;
                }
                let x = offset_x + obj.x;
                let y = offset_y - obj.y;
                transform.set_translation_z(-10.0);
                println!(
                    "### Adding collision object {}, x: {}, y: {}, width: {}, height: {} ###",
                    obj.name, x, y, obj.width, obj.height
                );
                bbox.position = Vector2::new(
                    offset_x + (obj.x * scale_x) + bbox.half_size.x,
                    offset_y - (obj.y * scale_y) - bbox.half_size.y,
                );
                bbox.old_position = bbox.position;
                let name = String::from(&obj.name);
                world
                    .create_entity()
                    .named(name)
                    .with(Motion::new())
                    .with(transform)
                    .with(collider)
                    .with(Direction::default())
                    .build();
            }
        }
    }

    fn load_tile_layer(
        &self,
        world: &mut World,
        layer: &Layer,
        sprite_sheet_handle: &Handle<SpriteSheet>,
    ) {
        println!("### Load tile layer: {} ###", layer.name);

        // TODO: support drawing in different directions
        // TODO: support different tileset spacing and margins
        // get the ssprite sheet handle
        if let Some(data) = &layer.data {
            for (index, d) in data.iter().enumerate() {
                let x = TILE_WIDTH / 2.0 + TILE_HEIGHT * (index % NUM_COLUMNS) as f32; // offset is half tile width
                let y = TILE_OFFSET_Y
                    - TILE_HEIGHT / 2.0
                    - (TILE_HEIGHT * (index / NUM_COLUMNS) as f32);

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
