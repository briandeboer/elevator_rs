use amethyst::{
    assets::{Asset, AssetStorage, Handle, Loader, ProcessingState},
    ecs::{VecStorage, World, WorldExt},
    renderer::{
        formats::texture::ImageFormat,
        Sprite, SpriteSheet, Texture,
        sprite::TextureCoordinates
    },
    error::Error,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tileset {
    columns: f32,
    image: String,
    imageheight: f32,
    imagewidth: f32,
    margin: f32,
    spacing: f32,
    tileheight: f32,
    tilewidth: f32,
    tilecount: f32,
}

impl From<Tileset> for Result<ProcessingState<Tileset>, Error> {
    fn from(tileset: Tileset) -> Result<ProcessingState<Tileset>, Error> {
        Ok(ProcessingState::Loaded(tileset))
    }
}

impl Asset for Tileset {
    const NAME: &'static str = "elevator_rs::Tileset";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Tileset>>;
}

impl Tileset {
    pub fn load_spritesheet(&self, world: &mut World) -> Handle<SpriteSheet> {
        println!("Loaded tileset with image: {}", self.image);

        // load 
        let texture_handle = {
            let loader = &world.fetch::<Loader>();
            let texture_storage = &world.fetch::<AssetStorage<Texture>>();
            loader.load(&self.image, ImageFormat::default(), (), &texture_storage)
        };

        let mut sprites: Vec<Sprite> = Vec::new();

        // TODO: should probably clear the vector first
        let rows = (self.tilecount / self.columns) as i32;
        let mut count = 0;

        println!("### columns: {}, rows: {}, tilecount: {} ###", self.columns, rows, self.tilecount);
        for y in 0..rows {
            for x in 0..(self.columns as i32) {
                
                // Coordinates of the 64x64 tile sprite inside the whole
                // tileset image, `terrainTiles_default.png` in this case
                // Important: TextureCoordinates Y axis goes from BOTTOM (0.0) to TOP (1.0)
                let tileset_sprite_columns = (self.imagewidth / self.tilewidth) as i32;
                let tileset_sprite_offset_columns = 1.0 / tileset_sprite_columns as f32;

                let tileset_sprite_rows = (self.imageheight / self.tileheight) as i32;
                let tileset_sprite_offset_rows = 1.0 / tileset_sprite_rows as f32;
                
                let tex_coords = TextureCoordinates {
                    left: x as f32 * tileset_sprite_offset_columns,
                    right: (x + 1) as f32 * tileset_sprite_offset_columns,
                    bottom: (y + 1) as f32 * tileset_sprite_offset_rows,
                    top: y as f32 * tileset_sprite_offset_rows
                };

                let sprite = Sprite {
                    width: self.tilewidth as f32,
                    height: self.tileheight as f32,
                    offsets: [0.0, 0.0],
                    tex_coords
                };

                sprites.push(sprite);
                count += 1;
            }
        }

        println!("### Sprites.len: {} ###", sprites.len());

        let sprite_sheet = SpriteSheet {
            texture: texture_handle,
            sprites: sprites,
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
    }

    // pub fn get_spritesheet(&self, texture_handle: Handle) -> SpriteSheet {
    //     // TODO: should probably clear the vector first
    //     let rows = self.columns / tile_count;

    //     // from other code
    //     let tileset_width = &map_tileset.images[0].width;
    //     let tileset_height = &map_tileset.images[0].height;
        
    //     for y in (0..rows).rev() {
    //         for x in 0..self.columns {
                
    //             // Coordinates of the 64x64 tile sprite inside the whole
    //             // tileset image, `terrainTiles_default.png` in this case
    //             // Important: TextureCoordinates Y axis goes from BOTTOM (0.0) to TOP (1.0)
    //             let tex_coords = TextureCoordinates {
    //                 left: (x * tile_width) as f32,
    //                 right: ((x + 1) * tile_width) as f32,
    //                 bottom: (y * tile_height) as f32,
    //                 top: ((y + 1) * tile_height) as f32,
    //             };

    //             let sprite = Sprite {
    //                 width: self.tile_width as f32,
    //                 height: self.tile_height as f32,
    //                 offsets: [0.0, 0.0],
    //                 tex_coords
    //             };

    //             self.sprites.push(sprite);
    //         }
    //     }

    //     SpriteSheet {
    //         texture: texture_handle,
    //         sprites: self.sprites,
    //     }
    // }
}