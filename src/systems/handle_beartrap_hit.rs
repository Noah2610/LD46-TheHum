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
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    );

    fn run(
        &mut self,
        (
            entities,
            beartrap_store,
            mut beartrap_affected_store,
            collider_store,
            mut animations_store,
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
                            movement:             beartrap
                                .crippled_movement
                                .clone(),
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
            if !beartrap_affected.is_crippled() {
                if let Some(beartrap_collision) = beartrap_affected_collider
                    .query::<FindQuery<CollisionTag>>()
                    .filter_ids(&beartrap_entity_ids)
                    .exp(&query_exp)
                    .run()
                {
                    if let Some(beartrap_data) =
                        beartraps_data.get(&beartrap_collision.id)
                    {
                        // MAKE PLAYER CRIPPLED
                        beartrap_affected.crippled_data =
                            Some(BeartrapAffectedCrippledData {
                                timer:    Timer::new(
                                    Some(
                                        Duration::from_millis(
                                            beartrap_data.crippled_duration_ms,
                                        )
                                        .into(),
                                    ),
                                    None,
                                ),
                                movement: beartrap_data.movement.clone(),
                            });

                        // PLAY BEARTRAP HIT ANIMATION (on beartrap)
                        let beartrap_entity =
                            entities.entity(beartrap_collision.id);
                        if let Some(beartrap_animations) =
                            animations_store.get_mut(beartrap_entity)
                        {
                            if let Err(e) = beartrap_animations
                                .push(AnimationKey::BeartrapHit)
                            {
                                eprintln!(
                                    "[WARNING]\n[HandleBeartrapHitSystem]\n    \
                                    Beartrap has no `BeartrapHit` animation \
                                    configured.\n    {}",
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

struct BeartrapData {
    crippled_duration_ms: u64,
    movement:             BeartrapAffectedMovementData,
}
