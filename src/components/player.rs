use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

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

// TODO: does all of this stuff really need to be public?
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
    pub is_ducking: bool,
    pub last_jump_state: bool,
    pub jump_time: Option<f32>,
    pub width: f32,
    pub height: f32,
    pub velocity: Vector2<f32>,
    pub max_ground_speed: f32,
    pub max_jump_velocity: f32,
    pub position: Vector2<f32>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            state: PlayerState::Idling,
            is_ducking: false,
            last_jump_state: false,
            jump_time: None,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            velocity: Vector2::new(0., 0.),
            max_ground_speed: 36.,
            max_jump_velocity: 90.,
            position: Vector2::new(0., 0.),
        }
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}
