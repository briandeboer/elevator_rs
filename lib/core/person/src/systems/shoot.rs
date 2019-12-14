use amethyst::ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage};

use crate::bullet::spawn_bullet;
use crate::components::{Gun, GunState, Person};
use asset::{AssetType, SpriteSheetList};
use hierarchy::components::Child;
use physics::components::Direction;

pub struct ShootSystem;

impl<'s> System<'s> for ShootSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Gun>,
        ReadStorage<'s, Child>,
        ReadStorage<'s, Person>,
        ReadStorage<'s, Direction>,
        ReadExpect<'s, SpriteSheetList>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut guns, children, persons, directions, sprite_sheet_list, lazy_update) =
            data;

        for (gun_entity, gun, child, direction) in
            (&entities, &mut guns, &children, &directions).join()
        {
            for (entity, person) in (&entities, &persons).join() {
                let parent = child.parent;
                if parent == entity
                    && !gun.spawned_bullet
                    && (gun.state == GunState::Shooting || gun.state == GunState::JumpShooting)
                {
                    let pos_x = person.position.x;
                    let pos_y = person.position.y;

                    let bullet_sprite_sheet_handle =
                        { sprite_sheet_list.get(AssetType::Bullet).unwrap().clone() };
                    spawn_bullet(
                        &entities,
                        gun_entity,
                        bullet_sprite_sheet_handle,
                        pos_x,
                        pos_y,
                        direction,
                        &lazy_update,
                    );
                    gun.shots_fired += 1;
                    gun.spawned_bullet = true;
                }
            }
        }
    }
}
