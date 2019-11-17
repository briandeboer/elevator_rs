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
    pub is_ducking: bool,
    pub jump_time: Option<f32>,
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
    pub max_ground_speed: f32,
    pub max_jump_velocity: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            state: PlayerState::Idling,
            is_shooting: false,
            is_ducking: false,
            jump_time: None,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            velocity: [0.0, 0.0],
            max_ground_speed: 40.,
            max_jump_velocity: 14.,
        }
    }
}
