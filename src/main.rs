mod components;
mod entities;
mod resources;
mod states;
mod systems;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystemDesc, Processor},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    Application, GameDataBuilder,
};
use components::{AnimationId, AnimationPrefabData};
use resources::{Map, Tileset};
use systems::{
    AnimationControlSystem, BulletCollisionSystem, BulletImpactAnimationSystem, CollisionSystem,
    ControlsSystem, DirectionSystem, GunAnimationSystem, KinematicsSystem, PlayerAnimationSystem,
    PlayerKinematicsSystem, ShootSystem,
};

fn main() -> amethyst::Result<()> {
    // start logging in amethyst
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let assets_dir = app_root.join("assets");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<AnimationPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Tileset>::new(), "tileset_processor", &[])
        .with(Processor::<Map>::new(), "map_processor", &[])
        .with(ControlsSystem, "controls_system", &[])
        .with(
            systems::ElevatorControlSystem,
            "elevator_control_system",
            &[],
        )
        .with(
            PlayerKinematicsSystem,
            "player_kinematics_system",
            &["controls_system"],
        )
        .with(
            KinematicsSystem,
            "kinematics_system",
            &["player_kinematics_system"],
        )
        .with(ShootSystem, "shoot_system", &["kinematics_system"])
        // PincerAi
        .with(CollisionSystem, "collision_system", &["shoot_system"])
        .with(
            BulletCollisionSystem,
            "bullet_collision_system",
            &["collision_system"],
        )
        // PincerCollision
        // MarineCollision
        .with(
            systems::PlayerTransformationSystem,
            "player_transformation_system",
            &[],
        )
        // .with(
        //     systems::GunTransformationSystem,
        //     "gun_transformation_system",
        //     &["transformation_system"],
        // )
        .with(
            systems::ElevatorTransformationSystem,
            "elevator_transformation_system",
            &["player_transformation_system"],
        )
        // BulletTransformation
        // BulletImpact
        .with(
            BulletImpactAnimationSystem,
            "bullet_impact_animation_system",
            &[],
        )
        .with(
            PlayerAnimationSystem,
            "player_animation_system",
            &[], // &["transformation_system"],
        )
        .with(
            GunAnimationSystem,
            "gun_animation_system",
            &[], // &["transformation_system"],
        )
        .with(
            AnimationControlSystem,
            "animation_control_system",
            &["player_animation_system", "gun_animation_system"],
        )
        .with(
            DirectionSystem,
            "direction_system",
            &["player_transformation_system"], //"gun_transformations_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game =
        Application::build(assets_dir, states::GameState::default())?.build(game_data)?;
    game.run();

    Ok(())
}
