use amethyst::ecs::{Component, DenseVecStorage};

// TODO: does all of this stuff really need to be public?
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Enemy {
    pub spawn_time: f64,
}

impl Enemy {
    pub fn new(spawn_time: f64) -> Enemy {
        Enemy { spawn_time }
    }
}
