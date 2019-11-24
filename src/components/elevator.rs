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
    pub boundary_top: f32,
    pub boundary_bottom: f32,
    pub num_floors: i32,
    pub velocity: f32,
    pub current_floor: i32,
    pub next_floor: i32,
}

impl Elevator {
    pub fn new(velocity: f32, boundary_top: f32, boundary_bottom: f32, num_floors: i32) -> Self {
        Elevator {
            boundary_top,
            boundary_bottom,
            num_floors,
            velocity,
            current_floor: 0,
            next_floor: 0,
         }
    }
}
