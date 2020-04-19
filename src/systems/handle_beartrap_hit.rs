use super::system_prelude::*;
use climer::Timer;
use deathframe::physics::query::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Default)]
pub struct HandleBeartrapHitSystem;

impl<'a> System<'a> for HandleBeartrapHitSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Beartrap>,
        WriteStorage<'a, BeartrapAffected>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            beartrap_store,
            mut beartrap_affected_store,
            collider_store,
        ): Self::SystemData,
    ) {
        let mut beartrap_entity_ids = Vec::new();
        let beartraps_data: HashMap<Index, BeartrapData> =
            (&entities, &beartrap_store)
                .join()
                .filter_map(|(entity, beartrap)| {
                    if beartrap.is_active {
                        let entity_id = entity.id();
                        beartrap_entity_ids.push(entity_id);
                        Some((entity_id, BeartrapData {
                            crippled_duration_ms: beartrap.crippled_duration_ms,
                        }))
                    } else {
                        None
                    }
                })
                .collect();

        let query_exp = {
            use deathframe::physics::query::exp::prelude_variants::*;

            And(vec![IsTag(CollisionTag::Beartrap), IsState(Enter)])
        };

        for (beartrap_affected, beartrap_affected_collider) in
            (&mut beartrap_affected_store, &collider_store).join()
        {
            if let Some(beartrap_collision) = beartrap_affected_collider
                .query::<FindQuery<CollisionTag>>()
                .filter_ids(&beartrap_entity_ids)
                .exp(&query_exp)
                .run()
            {
                if let Some(beartrap_data) =
                    beartraps_data.get(&beartrap_collision.id)
                {
                    beartrap_affected.is_crippled = true;
                    beartrap_affected.timer = Some(Timer::new(
                        Some(
                            Duration::from_millis(
                                beartrap_data.crippled_duration_ms,
                            )
                            .into(),
                        ),
                        None,
                    ));
                }
            }
        }
    }
}

struct BeartrapData {
    crippled_duration_ms: u64,
}
