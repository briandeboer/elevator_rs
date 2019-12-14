use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

const PERSON_HEIGHT: f32 = 16.0;
const PERSON_WIDTH: f32 = 16.0;

#[allow(dead_code)] // TODO: remove when all variants are finished
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PersonState {
    Ducking,
    Dying,
    Idling,
    Jumping,
    Shooting,
    Walking,
    Hopping,
    EnteringRoom,
    InsideRoom,
    ExitingRoom,
}

impl Default for PersonState {
    fn default() -> Self {
        PersonState::Idling
    }
}

// TODO: does all of this stuff really need to be public?
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Person {
    pub state: PersonState,
    pub width: f32,
    pub height: f32,
    pub velocity: Vector2<f32>,
    pub max_ground_speed: f32,
    pub max_jump_velocity: f32,
    pub position: Vector2<f32>,
    pub ride_velocity: Vector2<f32>,
}

impl Default for Person {
    fn default() -> Self {
        Self::new()
    }
}

impl Person {
    pub fn new() -> Person {
        Person {
            state: PersonState::Idling,
            width: PERSON_WIDTH,
            height: PERSON_HEIGHT,
            velocity: Vector2::new(0., 0.),
            max_ground_speed: 36.,
            max_jump_velocity: 110.,
            position: Vector2::new(0., 0.),
            ride_velocity: Vector2::new(0., 0.),
        }
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn update_ride_velocity(&mut self, x: f32, y: f32) {
        self.ride_velocity.x = x;
        self.ride_velocity.y = y;
    }
}
