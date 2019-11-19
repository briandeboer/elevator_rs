use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    core::{math::Vector2, transform::Transform, WithNamed},
    ecs::{Builder, VecStorage, World, WorldExt},
    error::Error,
    renderer::{SpriteRender, SpriteSheet},
};

use serde::{Deserialize, Serialize};

use crate::components::{Collider, Direction, Motion};

const Y_OFFSET: f32 = 150.0;
const TILE_SIZE: f32 = 8.0;
const NUM_COLUMNS: usize = 26;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub name: String,
    pub height: f32,
    pub width: f32,
    pub rotation: f32,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
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
                _ => {
                    self.load_tile_layer(world, layer, &sprite_sheet_handle);
                }
            }
        }
    }

    fn load_collision_layer(&self, world: &mut World, layer: &Layer) {
        let scale_x: f32 = 1.0;
        let scale_y: f32 = 1.0;
        let offset_x: f32 = 0.0;
        let offset_y: f32 = 150.;
        
        if let Some(objects) = &layer.objects {
            for (_index, obj) in objects.iter().enumerate() {
                let mut transform = Transform::default();
                let mut collider = Collider::new(obj.width * scale_x, obj.height * scale_y);
                let bbox = &mut collider.bounding_box;
                let x = offset_x + obj.x;
                let y = offset_y - obj.y;
                transform.set_translation_z(0.0);
                println!("### Adding collision object {}, x: {}, y: {}, width: {}, height: {} ###", obj.name, x, y, obj.width, obj.height);
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

    fn load_tile_layer(&self, world: &mut World, layer: &Layer, sprite_sheet_handle: &Handle<SpriteSheet>) {
        println!("### Load tile layer: {} ###", layer.name);

        // TODO: support drawing in different directions
        // TODO: support different tileset spacing and margins
        // get the ssprite sheet handle
        if let Some(data) = &layer.data {
            for (index, d) in data.iter().enumerate() {
                let tile_sprite = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: (*d - 1) as usize, // sprite numbers are off by 1 in t
                };
                let mut tile_transform = Transform::default();
                // TODO: make these not hardcoded
                let x = 4.0 + TILE_SIZE * (index % NUM_COLUMNS) as f32;
                let y = Y_OFFSET - (TILE_SIZE * (index / NUM_COLUMNS) as f32);
                tile_transform.set_translation_xyz(x, y, 0.0);
                world.create_entity()
                    .named("map_tile")
                    .with(tile_transform)
                    .with(tile_sprite)
                    .build();
            }
        }
    }
}