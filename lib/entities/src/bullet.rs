use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Named, Transform,
    },
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{sprite::SpriteSheetHandle, transparent::Transparent, SpriteRender},
};

use components::{
    Animation, AnimationId, AnimationPrefabData, Bullet, BulletImpact, Collidee, Collider,
    DefaultTransformation, Direction, Directions, Motion,
};

const SCALE: f32 = 1.0;
const OFFSET_X: f32 = 11.;
const OFFSET_Y: f32 = 3.;
const BULLET_WIDTH: f32 = 6.;
const BULLET_HEIGHT: f32 = 3.;
const BULLET_VELOCITY: f32 = 200.0;

pub fn spawn_bullet(
    entities: &Entities,
    gun_entity: Entity,
    sprite_sheet_handle: SpriteSheetHandle,
    shoot_start_position_x: f32,
    shoot_start_position_y: f32,
    shooter_direction: &Direction,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let bullet_entity: Entity = entities.create();
    let scale = SCALE;

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

    let bullet_start_position: f32 = match shooter_direction.x {
        Directions::Right => {
            motion.velocity.x = BULLET_VELOCITY;
            direction.x = Directions::Right;
            shoot_start_position_x + OFFSET_X
        }
        Directions::Left => {
            motion.velocity.x = -BULLET_VELOCITY;
            direction.x = Directions::Left;
            shoot_start_position_x - OFFSET_X
        }
        _ => 0.,
    };

    let mut collider = Collider::new(BULLET_WIDTH * scale, BULLET_HEIGHT * scale); // bullet width and height
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(bullet_start_position, shoot_start_position_y + OFFSET_Y);
    bbox.old_position = bbox.position;

    transform.set_translation_x(bullet_start_position);
    transform.set_translation_y(shoot_start_position_y + OFFSET_Y);
    transform.set_translation_z(0.5);

    lazy_update.insert(bullet_entity, Bullet::new(Some(gun_entity)));
    lazy_update.insert(bullet_entity, Named::new("Bullet"));
    lazy_update.insert(bullet_entity, collider);
    lazy_update.insert(bullet_entity, Collidee::default());
    lazy_update.insert(bullet_entity, DefaultTransformation::default());
    lazy_update.insert(bullet_entity, sprite_render);
    lazy_update.insert(bullet_entity, motion);
    lazy_update.insert(bullet_entity, transform);
    lazy_update.insert(bullet_entity, direction);
    lazy_update.insert(bullet_entity, Transparent);
}

pub fn show_bullet_impact(
    entities: &Entities,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    impact_position_x: f32,
    impact_position_y: f32,
    bullet_velocity: f32,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let bullet_impact_entity: Entity = entities.create();
    let scale = SCALE;

    let mut direction = Direction::new(
        Directions::Right,
        Directions::Neutral,
        Directions::Neutral,
        Directions::Neutral,
    );

    direction.x = if bullet_velocity > 0. {
        Directions::Right
    } else {
        Directions::Left
    };

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_x(impact_position_x);
    transform.set_translation_y(impact_position_y);
    transform.set_translation_z(1.0);

    lazy_update.insert(bullet_impact_entity, BulletImpact::default());
    lazy_update.insert(bullet_impact_entity, Named::new("BulletImpact"));
    lazy_update.insert(
        bullet_impact_entity,
        Animation::new(AnimationId::BulletImpact, vec![AnimationId::BulletImpact]),
    );
    lazy_update.insert(bullet_impact_entity, prefab_handle);
    lazy_update.insert(bullet_impact_entity, transform);
    lazy_update.insert(bullet_impact_entity, direction);
    lazy_update.insert(bullet_impact_entity, Transparent);
}
