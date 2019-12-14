use amethyst::ecs::{Component, DenseVecStorage};

// TODO: does all of this stuff really need to be public?
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Enemy {}

impl Default for Enemy {
    fn default() -> Self {
        Self::new()
    }
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {}
    }
}
