use super::system_prelude::*;
use rand::prelude::SliceRandom;

#[derive(Default)]
pub struct UpdateWoodSpawnerManagerSystem;

impl<'a> System<'a> for UpdateWoodSpawnerManagerSystem {
    type SystemData = (
        WriteExpect<'a, WoodSpawnerManager>,
        WriteStorage<'a, WoodSpawner>,
        ReadStorage<'a, Bonfire>,
        ReadStorage<'a, WoodInventory>,
    );

    fn run(
        &mut self,
        (
            mut wood_spawner_manager,
            mut wood_spawner_store,
            bonfire_store,
            wood_inventory_store,
        ): Self::SystemData,
    ) {
        if wood_spawner_manager.should_update_wood_spawners() {
            let mut rng = rand::thread_rng();

            let bonfire_woods = (&bonfire_store, &wood_inventory_store)
                .join()
                .next()
                .map(|(_, inventory)| inventory.woods)
                .unwrap_or(0);

            let active_percentage = (wood_spawner_manager.active_percentage
                - (bonfire_woods as f32
                    * wood_spawner_manager.decrease_active_percentage_step))
                .max(wood_spawner_manager.min_active_percentage);
            let wood_spawners_amount_to_activate =
                (wood_spawner_store.count() as f32 * active_percentage)
                    as usize;
            let mut shuffled_wood_spawners = (&mut wood_spawner_store)
                .join()
                .collect::<Vec<&mut WoodSpawner>>(
            );
            shuffled_wood_spawners.shuffle(&mut rng);
            for (i, wood_spawner) in
                shuffled_wood_spawners.into_iter().enumerate()
            {
                wood_spawner.set_active(i < wood_spawners_amount_to_activate);
            }
        }
    }
}
