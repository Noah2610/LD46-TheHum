use crate::resource;
use crate::resources::prelude::*;
use crate::settings::Settings;
use crate::states::aliases::{CustomData, GameDataBuilder};
use deathframe::amethyst;

pub(super) fn build_game_data<'a, 'b>(
    settings: &Settings,
) -> amethyst::Result<GameDataBuilder<'a, 'b>> {
    use crate::input::prelude::*;
    use crate::systems::prelude::*;
    use amethyst::core::transform::TransformBundle;
    use amethyst::renderer::types::DefaultBackend;
    use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
    use amethyst::ui::{RenderUi, UiBundle};
    use amethyst::utils::fps_counter::FpsCounterBundle;
    use amethyst::utils::ortho_camera::CameraOrthoSystem;
    use deathframe::bundles::*;

    let transform_bundle = TransformBundle::new();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(resource("config/display.ron"))?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderUi::default())
        .with_plugin(RenderFlat2D::default());
    let audio_bundle = AudioBundle::<SoundKey, SongKey>::default();
    let menu_input_bundle = MenuBindings::bundle()?;
    let ingame_input_bundle = IngameBindings::bundle()?;
    let physics_bundle =
        PhysicsBundle::<CollisionTag, SolidTag>::new().with_deps(&[]);
    let animation_bundle = AnimationBundle::<AnimationKey>::new();
    let reactive_animation_bundle =
        AnimationBundle::<ReactiveAnimationKey>::new()
            .with_name_suffix("_reactive");

    let custom_game_data = GameDataBuilder::default()
        .custom(CustomData::default())
        .dispatcher(DispatcherId::MainMenu)?
        .dispatcher(DispatcherId::Ingame)?
        .dispatcher(DispatcherId::Paused)?
        .with_core_bundle(FpsCounterBundle)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(audio_bundle)?
        .with_core_bundle(menu_input_bundle)?
        .with_core_bundle(UiBundle::<MenuBindings>::new())?
        .with_core(PrintFpsSystem::default(), "print_fps_system", &[])?
        .with_core(CameraOrthoSystem::default(), "camera_ortho_system", &[])?
        .with_core(ScaleSpritesSystem::default(), "scale_sprites_system", &[])?
        .with_core(
            InputManagerSystem::<MenuBindings>::default(),
            "menu_input_manager_system",
            &[],
        )?
        .with_bundle(DispatcherId::Ingame, ingame_input_bundle)?
        .with_bundle(DispatcherId::Ingame, physics_bundle)?
        .with_bundle(DispatcherId::Ingame, animation_bundle)?
        .with_bundle(DispatcherId::Ingame, reactive_animation_bundle)?
        .with(
            DispatcherId::Ingame,
            InputManagerSystem::<IngameBindings>::default(),
            "ingame_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Paused,
            InputManagerSystem::<MenuBindings>::default(),
            "paused_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            FollowSystem::default(),
            "follow_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ConfineEntitiesSystem::default(),
            "confine_entities_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            EntityLoaderSystem::default(),
            "entity_loader_system",
            &[
                "move_entities_system",
                "follow_system",
                "confine_entities_system",
            ],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateHealthSystem::default(),
            "update_health_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateLifecycleSystem::default(),
            "update_lifecycle_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ControlPlayerSystem::default(),
            "control_player_system",
            &["ingame_input_manager_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleMovablesSystem::default(),
            "handle_movables_system",
            &["control_player_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleFlameVisibilitySystem::default(),
            "handle_flame_visibility_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            UpdatePlayerAnimationSystem::default(),
            "update_player_animation_system",
            &["control_player_system"],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateReactiveAnimationsSystem::default(),
            "update_reactive_animations_system",
            &["update_collisions_system"],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateWoodSpawnerManagerSystem::default(),
            "update_wood_spawner_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            SpawnWoodSystem::default(),
            "spawn_wood_system",
            &["update_wood_spawner_manager_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandlePlayerWoodPickupSystem::default(),
            "handle_player_wood_pickup_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateWoodInventorySystem::default(),
            "update_wood_inventory_system",
            &["handle_player_wood_pickup_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandlePlayerFeedBonfireSystem::default(),
            "handle_player_feed_bonfire_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateFlameRadiusSystem::default(),
            "update_flame_radius_system",
            &["update_wood_inventory_system"],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateBonfireHaloSizeSystem::default(),
            "update_bonfire_halo_size_system",
            &["update_flame_radius_system"],
        )?
        .with(
            DispatcherId::Ingame,
            PlayRandomSoundsSystem::default(),
            "play_random_sounds_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleLadderClimbingSystem::default(),
            "handle_ladder_climbing_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            DecreaseBonfireFlameSystem::default(),
            "decrease_bonfire_flame_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            SpawnBeartrapSystem::default(),
            "spawn_beartrap_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleBeartrapHitSystem::default(),
            "handle_beartrap_hit_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleBeartrapAffectedSystem::default(),
            "handle_beartrap_affected_system",
            &["handle_beartrap_hit_system"],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateBonfireSongVolumeSystem::new(
                settings.general.bonfire_song_volume_factor,
            ),
            "update_bonfire_song_volume",
            &[],
        )?
        // - comment for easier copy/pasting -
        ;

    Ok(custom_game_data)
}
