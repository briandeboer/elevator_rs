use amethyst::{
    assets::{Handle, Prefab},
    core::math::Vector2,
    core::transform::Transform,
    core::Named,
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
};

use crate::components::Enemy;

use animation::components::{Animation, AnimationId, AnimationPrefabData};
use hierarchy::components::Child;
use person::components::{Gun, Person};
use physics::components::{Collidee, Collider, Direction, Directions, Motion, Proximity};

const ENEMY_Z: f32 = 0.5;

/// Initialises one player in the middle-ish space
pub fn spawn_enemy(
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
    enemy_prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    guns_prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    position: Vector2<f32>,
) {
    let mut transform = Transform::default();
    transform.set_translation_z(ENEMY_Z);

    let mut collider = Collider::new(12., 24.);
    collider.allow_proximity = true;
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(position.x + bbox.half_size.x, position.y - bbox.half_size.y);
    bbox.old_position = bbox.position;

    let enemy_entity: Entity = entities.create();
    lazy_update.insert(enemy_entity, Named::new("Enemy"));
    lazy_update.insert(enemy_entity, Enemy::new());
    lazy_update.insert(enemy_entity, Person::new());
    lazy_update.insert(enemy_entity, collider);
    lazy_update.insert(enemy_entity, Collidee::default());
    lazy_update.insert(enemy_entity, transform);
    lazy_update.insert(
        enemy_entity,
        Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Die,
                AnimationId::Hop,
                AnimationId::Jump,
                AnimationId::Idle,
                AnimationId::Walk,
                AnimationId::Duck,
            ],
        ),
    );
    lazy_update.insert(enemy_entity, enemy_prefab_handle);
    lazy_update.insert(enemy_entity, Motion::new());
    lazy_update.insert(
        enemy_entity,
        Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ),
    );
    lazy_update.insert(enemy_entity, Proximity::default());

    let mut gun_transform = Transform::default();
    gun_transform.set_translation_xyz(position.x, position.y, ENEMY_Z + 0.2);
    let gun_entity: Entity = entities.create();
    lazy_update.insert(gun_entity, Named::new("Gun"));
    lazy_update.insert(gun_entity, Child::new(enemy_entity, 8., 2., 0.));
    lazy_update.insert(gun_entity, Gun::new(false));
    lazy_update.insert(gun_entity, gun_transform);
    lazy_update.insert(
        gun_entity,
        Animation::new(
            AnimationId::Holster,
            vec![
                AnimationId::PersonShoot,
                AnimationId::PersonJumpShoot,
                AnimationId::Holster,
            ],
        ),
    );
    lazy_update.insert(gun_entity, guns_prefab_handle);
    lazy_update.insert(
        gun_entity,
        Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ),
    );
}
