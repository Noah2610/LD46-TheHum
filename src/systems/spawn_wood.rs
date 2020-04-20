use super::system_prelude::*;
use crate::entities::{init_wood_with_storages, InitWoodStorages};
use crate::resource;

#[derive(Default)]
pub struct SpawnWoodSystem;

impl<'a> System<'a> for SpawnWoodSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WoodSpawner>,
        InitWoodStorages<'a>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, WoodIndicator>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut wood_spawner_store,
            mut init_wood_storages,
            mut animation_store,
            mut wood_indicator_store,
        ): Self::SystemData,
    ) {
        let is_spawned_food_alive = |wood_spawner: &WoodSpawner| -> bool {
            wood_spawner
                .spawned_wood_entity
                .map(|spawned_entity| entities.is_alive(spawned_entity))
                .unwrap_or(false)
        };

        let mut woods_to_spawn: Vec<(Entity, Transform)> = Vec::new();

        for (wood_spawner_transform, wood_spawner) in
            (&init_wood_storages.transform_store, &mut wood_spawner_store)
                .join()
        {
            if wood_spawner.should_spawn_wood()
                && !is_spawned_food_alive(wood_spawner)
            {
                let new_wood = entities.create();
                wood_spawner.spawned_wood_entity = Some(new_wood);
                woods_to_spawn.push((new_wood, wood_spawner_transform.clone()));
            }
        }

        for wood_to_spawn in woods_to_spawn {
            // INIT WOOD
            init_wood_with_storages(
                wood_to_spawn.0,
                wood_to_spawn.1.clone(),
                &mut init_wood_storages,
            )
            .expect("Couldn't spawn wood from WoodSpawner in SpawnWoodSystem");

            // INIT WOOD INDICATOR
            if let Some(sprite_sheet) = init_wood_storages
                .sprite_sheet_handles
                .get(&resource("spritesheets/wood_indicator.png"))
            {
                let sprite_render = SpriteRender {
                    sprite_sheet,
                    sprite_number: 0,
                };

                let _indicator = entities
                    .build_entity()
                    .with(
                        // Transform
                        wood_to_spawn.1,
                        &mut init_wood_storages.transform_store,
                    )
                    .with(
                        // Size
                        init_wood_storages.settings.wood.indicator.size.clone(),
                        &mut init_wood_storages.size_store,
                    )
                    .with(
                        sprite_render,
                        &mut init_wood_storages.sprite_render_store,
                    )
                    .with(
                        ScaleOnce::default(),
                        &mut init_wood_storages.scale_once_store,
                    )
                    .with(
                        Transparent,
                        &mut init_wood_storages.transparent_store,
                    )
                    .with(WoodIndicator::default(), &mut wood_indicator_store)
                    .with(
                        // Animation
                        {
                            let mut anim = init_wood_storages
                                .settings
                                .wood
                                .indicator
                                .animation
                                .clone();
                            anim.play_once();
                            anim
                        },
                        &mut animation_store,
                    )
                    .build();
            } else {
                eprintln!(
                    "[WARNING]\n[SpawnWoodSystem]\n    WoodIndicator \
                     spritesheet is not loaded"
                );
            }
        }
    }
}
