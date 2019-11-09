use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Player, PLAYER_WIDTH};
use crate::states::{GAME_HEIGHT, GAME_WIDTH};

/// Initialises one player in the middle-ish space
pub fn load_player(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the player in the middle for now.
    let y = GAME_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PLAYER_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(GAME_WIDTH - PLAYER_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // player is the first sprite in the sprite_sheet
    };

    // Create a player entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Player::new())
        .with(left_transform)
        .build();

}