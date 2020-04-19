use super::system_prelude::*;
use climer::Timer;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Default)]
pub struct DecreaseBonfireFlameSystem {
    timers: HashMap<Entity, Timer>,
}

impl<'a> System<'a> for DecreaseBonfireFlameSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Bonfire>,
        WriteStorage<'a, Flame>,
        ReadStorage<'a, WoodInventory>,
    );

    fn run(
        &mut self,
        (
            entities,
            bonfire_store,
            mut flame_store,
            wood_inventory_store,
        ): Self::SystemData,
    ) {
        for (entity, bonfire, flame, inventory) in (
            &entities,
            &bonfire_store,
            &mut flame_store,
            &wood_inventory_store,
        )
            .join()
        {
            let timer = self.timers.entry(entity).or_insert_with(|| {
                Timer::new(
                    Some(
                        Duration::from_millis(
                            bonfire.flame_decrease.interval_ms,
                        )
                        .into(),
                    ),
                    None,
                )
            });

            if timer.state.is_stopped() || timer.state.is_finished() {
                timer.start().unwrap();
            }
            timer.update().unwrap();

            if timer.state.is_finished() {
                flame.radius = (flame.radius
                    - (bonfire.flame_decrease.step
                        * (inventory.woods as f32
                            * bonfire.flame_decrease.woods_mult)))
                    .max(flame.min_radius);
            }
        }
    }
}
