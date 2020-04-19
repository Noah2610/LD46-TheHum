use super::system_prelude::*;
use crate::entities::{init_wood_with_storages, InitWoodStorages};

#[derive(Default)]
pub struct SpawnWoodSystem;

impl<'a> System<'a> for SpawnWoodSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WoodSpawner>,
        InitWoodStorages<'a>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut wood_spawner_store,
            mut init_wood_storages,
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
            init_wood_with_storages(
                wood_to_spawn.0,
                wood_to_spawn.1,
                &mut init_wood_storages,
            )
            .expect("Couldn't spawn wood from WoodSpawner in SpawnWoodSystem");
        }
    }
}
