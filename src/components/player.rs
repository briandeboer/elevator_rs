use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const PLAYER_HEIGHT: f32 = 16.0;
pub const PLAYER_WIDTH: f32 = 16.0;

pub enum MovementState {
    // Walking,
    Standing,
    // Jumping,
    // Ducking,
}

pub struct Player {
    pub movement_state: MovementState,
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

impl Player {
    pub fn new() -> Player {
        Player {
            movement_state: MovementState::Standing,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            velocity: [0.0, 0.0],
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
