use super::system_prelude::*;
use rand::prelude::SliceRandom;

#[derive(Default)]
pub struct UpdateWoodSpawnerManagerSystem;

impl<'a> System<'a> for UpdateWoodSpawnerManagerSystem {
    type SystemData = (
        WriteExpect<'a, WoodSpawnerManager>,
        WriteStorage<'a, WoodSpawner>,
    );

    fn run(
        &mut self,
        (mut wood_spawner_manager, mut wood_spawner_store): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();

        if wood_spawner_manager.should_update_wood_spawners() {
            let wood_spawners_amount_to_activate = (wood_spawner_store.count()
                as f32
                * wood_spawner_manager.active_percentage)
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
