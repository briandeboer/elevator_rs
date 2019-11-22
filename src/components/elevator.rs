use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct ElevatorTop {
    pub parent: Entity,
}

impl ElevatorTop {
    pub fn new(parent: Entity) -> Self {
        ElevatorTop {
            parent,
        }
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct ElevatorBottom {
    pub parent: Entity,
}

impl ElevatorBottom {
    pub fn new(parent: Entity) -> Self {
        ElevatorBottom {
            parent,
        }
    }
}

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Elevator {
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Elevator {
    pub fn new() -> Self {
        Elevator {
            pos_x: 0.,
            pos_y: 0.,
        }
    }
}
