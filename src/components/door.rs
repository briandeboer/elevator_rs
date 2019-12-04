use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage, NullStorage},
};

#[allow(dead_code)] // TODO: remove when all variants are finished
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum DoorState {
    Closed,
    Open,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Door {
    pub position: Vector2<f32>,
    pub can_user_enter: bool,
    pub state: DoorState,
    pub has_papers: bool,
}

impl Door {
    pub fn new(position: Vector2<f32>, can_user_enter: bool) -> Door {
        let has_papers = can_user_enter;
        Door {
            position,
            can_user_enter: false,
            state: DoorState::Closed,
            has_papers,
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct DoorEntry {}

