use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage, Entity},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Gun {
    pub parent: Option<Entity>,
    pub position_offset: Vector2<f32>,
    pub shots_fired: i32,
}

impl Default for Gun {
    fn default() -> Self {
        Self {
            parent: None,
            position_offset: Vector2::new(0., 0.),
            shots_fired: 0,
        }
    }
}

impl Gun {
    pub fn new(parent: Option<Entity>, offset_x: f32, offset_y: f32) -> Gun {
        Gun {
            parent: parent,
            position_offset: Vector2::new(offset_x, offset_y),
            shots_fired: 0,
        }
    }
}
