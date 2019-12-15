use amethyst::ecs::{Component, DenseVecStorage, Entity};

// TODO: does all of this stuff really need to be public?
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Enemy {
    pub spawn_time: f64,
    pub pursuing_entity: Option<Entity>,
    pub pursuit_time: f64,
}

impl Enemy {
    pub fn new(spawn_time: f64) -> Enemy {
        Enemy {
            spawn_time,
            pursuing_entity: None,
            pursuit_time: 0.,
        }
    }
}
