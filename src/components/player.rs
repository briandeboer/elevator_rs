use amethyst::ecs::{Component, DenseVecStorage};

pub const PLAYER_HEIGHT: f32 = 16.0;
pub const PLAYER_WIDTH: f32 = 16.0;

#[allow(dead_code)] // remove when all variants are finished
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PlayerState {
    Ducking,
    Dying,
    Idling,
    Jumping,
    Shooting,
    Walking,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
    pub is_shooting: bool,
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

impl Player {
    pub fn new() -> Player {
        Player {
            state: PlayerState::Idling,
            is_shooting: false,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            velocity: [0.0, 0.0],
        }
    }
}
