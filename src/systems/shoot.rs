use amethyst::{
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::components::{Direction, Gun, GunState, Player};
use crate::entities::spawn_bullet;
use crate::resources::{AssetType, SpriteSheetList};

pub struct ShootSystem;

impl<'s> System<'s> for ShootSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Gun>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Direction>,
        ReadExpect<'s, SpriteSheetList>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut guns, players, directions, sprite_sheet_list, lazy_update) = data;

        for (gun, direction) in (&mut guns, &directions).join() {
            for (entity, player) in (&entities, &players).join() {
                if let Some(parent) = gun.parent {
                    if parent == entity {
                        if !gun.spawned_bullet && (gun.state == GunState::Shooting || gun.state == GunState::JumpShooting) {
                            println!("Spawning a bullet!");
                            let pos_x = player.pos_x;
                            let pos_y = player.pos_y;
            
                            let bullet_sprite_sheet_handle =
                                { sprite_sheet_list.get(AssetType::Bullet).unwrap().clone() };
                            spawn_bullet(
                                &entities,
                                bullet_sprite_sheet_handle,
                                pos_x,
                                pos_y,
                                direction,
                                &lazy_update,
                            );
            
                            gun.spawned_bullet = true;
                        }
                    }
                }
            }
        }
    }
}