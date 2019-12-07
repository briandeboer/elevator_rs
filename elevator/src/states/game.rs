use amethyst::{
    assets::{AssetStorage, Handle, JsonFormat, Loader, ProgressCounter},
    prelude::*,
    ui::UiCreator,
};

use asset::{load_assets, AssetType, PrefabList, SpriteSheetList};
use camera::init_camera;
use elevator::load_elevator;
use map::{Map, Tileset};
use player::load_player;

/// state struct for the game state
#[derive(Default)]
pub struct GameState {
    progress_counter: Option<ProgressCounter>,
    map_handle: Option<Handle<Map>>,
    tileset_handle: Option<Handle<Tileset>>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("### GameState on_start ###");
        let world = data.world;

        self.progress_counter = Some(load_assets(
            world,
            vec![
                AssetType::Player,
                AssetType::Guns,
                AssetType::Bullet,
                AssetType::BulletImpact,
                AssetType::Door,
                AssetType::Elevator,
            ],
        ));

        let mut progress = ProgressCounter::default();
        world.exec(|mut creator: UiCreator<'_>| creator.create("ui/fps.ron", &mut progress));

        self.map_handle = {
            let loader = world.read_resource::<Loader>();
            Some(loader.load(
                "tilesets/level_1.json",
                JsonFormat,
                self.progress_counter.as_mut().expect("map"),
                &world.read_resource::<AssetStorage<Map>>(),
            ))
        };

        self.tileset_handle = {
            let loader = world.read_resource::<Loader>();
            Some(loader.load(
                "tilesets/tileset.json",
                JsonFormat,
                self.progress_counter.as_mut().expect("tileset"),
                &world.read_resource::<AssetStorage<Tileset>>(),
            ))
        };

        init_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            // Check if all data has been loaded
            if progress_counter.is_complete() {
                println!("### GameState progress complete ###");
                // Get the map, which is loaded in the on_start function of load state.

                let tileset = {
                    let tileset_storage = &data.world.read_resource::<AssetStorage<Tileset>>();
                    let tileset_handle = &self.tileset_handle.take().unwrap();
                    tileset_storage.get(tileset_handle).unwrap().clone()
                };
                let sprite_sheet = tileset.load_spritesheet(data.world);

                let map = {
                    let map_storage = &data.world.read_resource::<AssetStorage<Map>>();
                    let map_handle = &self.map_handle.take().unwrap();
                    map_storage.get(map_handle).unwrap().clone()
                };
                map.load_layers(data.world, sprite_sheet);

                let player_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Player).unwrap().clone()
                };
                let guns_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Guns).unwrap().clone()
                };
                println!("### Loading player ###");
                load_player(data.world, player_prefab_handle, guns_prefab_handle);

                println!("### Loading elevator ###");
                let elevator_sprite_sheet_handle = {
                    let sprite_sheet_list = data.world.read_resource::<SpriteSheetList>();
                    sprite_sheet_list.get(AssetType::Elevator).unwrap().clone()
                };
                load_elevator(data.world, elevator_sprite_sheet_handle);
                self.progress_counter = None;
            } else {
                println!(
                    "Loading: {}, Failed: {}, Finished: {}, Errors: {:?}",
                    progress_counter.num_loading(),
                    progress_counter.num_failed(),
                    progress_counter.num_finished(),
                    progress_counter.errors()
                );
            }
        }
        Trans::None
    }
}
