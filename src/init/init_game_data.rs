use crate::resource;
use crate::resources::prelude::*;
use crate::settings::Settings;
use crate::states::aliases::{CustomData, GameDataBuilder};
use deathframe::amethyst;

pub(super) fn build_game_data<'a, 'b>(
    _settings: &Settings,
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
        .with_bundle(DispatcherId::Ingame, ingame_input_bundle)?
        .with_bundle(DispatcherId::Ingame, physics_bundle)?
        .with_bundle(DispatcherId::Ingame, animation_bundle)?
        .with(
            DispatcherId::MainMenu,
            InputManagerSystem::<MenuBindings>::default(),
            "main_menu_input_manager_system",
            &[],
        )?
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
        )?;

    Ok(custom_game_data)
}
