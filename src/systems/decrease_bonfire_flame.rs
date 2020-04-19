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
    );

    fn run(
        &mut self,
        (entities, bonfire_store, mut flame_store): Self::SystemData,
    ) {
        for (entity, bonfire, flame) in
            (&entities, &bonfire_store, &mut flame_store).join()
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
                flame.radius = (flame.radius - bonfire.flame_decrease.step)
                    .max(flame.min_radius);
            }
        }
    }
}
