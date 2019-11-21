use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Named, Transform,
    },
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{sprite::SpriteSheetHandle, SpriteRender, transparent::Transparent},
};

use crate::{
    components::{Bullet, Collider, Collidee, Direction, Directions, Motion},
};

const OFFSET_X: f32 = 11.;
const OFFSET_Y: f32 = 3.;
const BULLET_WIDTH: f32 = 6.;
const BULLET_HEIGHT: f32 = 3.;
const BULLET_VELOCITY: f32 = 140.0;

pub fn spawn_bullet(
    entities: &Entities,
    sprite_sheet_handle: SpriteSheetHandle,
    shoot_start_position_x: f32,
    shoot_start_position_y: f32,
    shooter_direction: &Direction,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let bullet_entity: Entity = entities.create();
    let scale = 1.0;

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    let mut motion = Motion::new();

    let mut direction = Direction::new(
        Directions::Right,
        Directions::Neutral,
        Directions::Neutral,
        Directions::Neutral,
    );

    let bullet_start_position: f32 = 
        match shooter_direction.x {
            Directions::Right => {
                motion.velocity.x = BULLET_VELOCITY;
                direction.x = Directions::Right;
                shoot_start_position_x + OFFSET_X
            },
            Directions::Left => {
                motion.velocity.x = -BULLET_VELOCITY;
                direction.x = Directions::Left;
                shoot_start_position_x - OFFSET_X
            },
            _ => { 0. }
        };

    let mut collider = Collider::new(BULLET_WIDTH * scale, BULLET_HEIGHT * scale); // bullet width and height
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(bullet_start_position, shoot_start_position_y + OFFSET_Y);
    bbox.old_position = bbox.position;

    transform.set_translation_x(bullet_start_position);
    transform.set_translation_y(shoot_start_position_y + OFFSET_Y);
    transform.set_translation_z(0.5);

    lazy_update.insert(bullet_entity, Bullet::default());
    lazy_update.insert(bullet_entity, Named::new("Bullet"));
    lazy_update.insert(bullet_entity, collider);
    lazy_update.insert(bullet_entity, Collidee::default());
    lazy_update.insert(bullet_entity, sprite_render);
    lazy_update.insert(bullet_entity, motion);
    lazy_update.insert(bullet_entity, transform);
    lazy_update.insert(bullet_entity, direction);
    lazy_update.insert(bullet_entity, Transparent);
}