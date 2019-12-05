use amethyst::{core::transform::Transform, prelude::*, renderer::Camera};

// TODO: move these to a resource
pub const GAME_WIDTH: f32 = 208.0;
pub const GAME_HEIGHT: f32 = 200.0;

/// sets up a camera for the purposes of seeing the 2d space
pub fn init_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 10.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
        .with(transform)
        .build();
}
