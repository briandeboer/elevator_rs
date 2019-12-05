use amethyst::ecs::{Component, DenseVecStorage};

#[allow(dead_code)] // remove when all variants are finished
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum GunState {
    Shooting,
    JumpShooting,
    Holstered,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Gun {
    pub shots_fired: i32,
    pub state: GunState,
    pub is_player: bool,
    pub last_shoot_state: bool,
    pub last_shot_seconds: f64,
    pub spawned_bullet: bool,
}

impl Gun {
    pub fn new(is_player: bool) -> Gun {
        Gun {
            shots_fired: 0,
            state: GunState::Holstered,
            is_player,
            last_shoot_state: false,
            last_shot_seconds: -1.0,
            spawned_bullet: false,
        }
    }
}
