use amethyst::ecs::{Component, DenseVecStorage};

// TODO: does all of this stuff really need to be public?
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub player_number: usize,
    pub is_ducking: bool,
    pub last_jump_state: bool,
    pub jump_time: Option<f32>,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            player_number: 1,
            is_ducking: false,
            last_jump_state: false,
            jump_time: None,
        }
    }
}
