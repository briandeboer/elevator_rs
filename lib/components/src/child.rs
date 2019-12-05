use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Child {
    pub parent: Entity,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
}

impl Child {
    pub fn new(parent: Entity, offset_x: f32, offset_y: f32, offset_z: f32) -> Self {
        Child {
            parent,
            offset_x,
            offset_y,
            offset_z,
        }
    }
}
