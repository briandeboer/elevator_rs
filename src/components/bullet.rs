use amethyst::ecs::{Component, DenseVecStorage, Entity, NullStorage};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct BulletImpact;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Bullet {
    pub parent: Option<Entity>,
}

impl Bullet {
    pub fn new(parent: Option<Entity>) -> Self {
        Bullet {
            parent,
        }
    }
}
