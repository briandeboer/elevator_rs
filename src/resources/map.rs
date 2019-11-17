use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    core::transform::Transform,
    ecs::{Builder, VecStorage, World, WorldExt},
    error::Error,
    renderer::{SpriteRender, SpriteSheet},
};

use serde::{Deserialize, Serialize};

// TODO: need to include the crates we need

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub name: String,
    pub height: i32,
    pub width: i32,
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
                    // self.load_collision_layer(world, layer, ctx);
                }
                _ => {
                    self.load_tile_layer(world, layer, &sprite_sheet_handle);
                }
            }
        }
    }

    fn load_tile_layer(&self, world: &mut World, layer: &Layer, sprite_sheet_handle: &Handle<SpriteSheet>) {
        println!("### Load tile layer: {} ###", layer.name);

        // TODO: support drawing in different directions
        // get the ssprite sheet handle
        if let Some(data) = &layer.data {
            for (index, d) in data.iter().enumerate() {
                let tile_sprite = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: (*d - 1) as usize, // sprite numbers are off by 1 in t
                };
                let mut tile_transform = Transform::default();
                let x = 8.0 * (index % 26) as f32;
                let y = 150.0 - (8.0 * (index / 26) as f32);
                println!("### Tile: {}, sprite number: {}, x: {}, y: {} ###", index, d, x, y);
                tile_transform.set_translation_xyz(x, y, 0.0);
                world.create_entity()
                    .with(tile_transform)
                    .with(tile_sprite)
                    .build();
            }
        }
    }
}