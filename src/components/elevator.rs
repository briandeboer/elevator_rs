use amethyst::{
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, NullStorage},
};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Rideable;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct ElevatorComponent {
    pub name: &'static str,
    pub sprite_number: usize,
    pub width: f32,
    pub height: f32,
    pub offsets: Vector3<f32>,
    pub is_collidable: bool,
}

impl ElevatorComponent {
    pub fn new(
        name: &'static str,
        sprite_number: usize,
        width: f32,
        height: f32,
        offsets: Vector3<f32>,
        is_collidable: bool,
    ) -> Self {
        ElevatorComponent {
            name,
            sprite_number,
            width,
            height,
            offsets,
            is_collidable,
        }
    }
}

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Elevator {
    pub velocity: f32,
}

impl Elevator {
    pub fn new(velocity: f32) -> Self {
        Elevator { velocity }
    }
}
