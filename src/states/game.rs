use amethyst::{assets::ProgressCounter, prelude::*};

use crate::components::*;
use crate::entities::{init_camera, load_player};
use crate::resources::{load_assets, AssetType, Context};

// TODO: move these to a resource
pub const GAME_WIDTH: f32 = 200.0;
pub const GAME_HEIGHT: f32 = 200.0;

/// state struct for the game state
#[derive(Default)]
pub struct GameState {
    progress_counter: Option<ProgressCounter>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("### GameState on_start ###");
        let world = data.world;

        world.insert(Context::new());
        // needed until systems are done
        world.register::<Player>();

        self.progress_counter = Some(load_assets(world, vec![AssetType::Player]));
        init_camera(world);
        // load_player(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            // Check if all data has been loaded
            if progress_counter.is_complete() {
                println!("### GameState progress complete ###");
                self.progress_counter = None;
            } else {
                println!("Loading: {}, Failed: {}, Finished: {}, Errors: {:?}",
                    progress_counter.num_loading(),
                    progress_counter.num_failed(),
                    progress_counter.num_finished(),
                    progress_counter.errors());
            }

        }
        Trans::None
    }
}

// fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
//     // Load the sprite sheet necessary to render the graphics.
//     // The texture is the pixel data
//     // `texture_handle` is a cloneable reference to the texture
//     let texture_handle = {
//         let loader = world.read_resource::<Loader>();
//         let texture_storage = world.read_resource::<AssetStorage<Texture>>();
//         loader.load(
//             "texture/game_spritesheet.png",
//             ImageFormat::default(),
//             (),
//             &texture_storage,
//         )
//     };

//     let loader = world.read_resource::<Loader>();
//     let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
//     loader.load(
//         "texture/game_spritesheet.ron",
//         SpriteSheetFormat(texture_handle),
//         (),
//         &sprite_sheet_store,
//     )
// }
