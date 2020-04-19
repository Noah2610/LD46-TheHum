use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleBeartrapAffectedSystem {
    normal_movements: HashMap<Entity, BeartrapAffectedMovementData>,
}

impl<'a> System<'a> for HandleBeartrapAffectedSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, BeartrapAffected>,
        WriteStorage<'a, Movable>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut beartrap_affected_store,
            mut movable_store,
        ): Self::SystemData,
    ) {
        for (entity, beartrap_affected, movable) in
            (&entities, &mut beartrap_affected_store, &mut movable_store).join()
        {
            let mut should_stop_being_crippled = false;

            if let Some(crippled_data) =
                beartrap_affected.crippled_data.as_mut()
            {
                let timer = &mut crippled_data.timer;

                if timer.state.is_stopped() {
                    // start being crippled
                    self.normal_movements.insert(
                        entity,
                        BeartrapAffectedMovementData {
                            acceleration: movable.data.acceleration,
                            max_velocity: movable.data.max_velocity,
                        },
                    );
                    movable.data.acceleration =
                        crippled_data.movement.acceleration;
                    movable.data.max_velocity =
                        crippled_data.movement.max_velocity;
                    timer.start().unwrap();
                }
                timer.update().unwrap();

                if timer.state.is_finished() {
                    // stop being crippled
                    let normal_movement =
                        self.normal_movements.remove(&entity).expect(
                            "Should have normal movement data when stop being \
                             crippled",
                        );
                    movable.data.acceleration = normal_movement.acceleration;
                    movable.data.max_velocity = normal_movement.max_velocity;
                    should_stop_being_crippled = true;
                }
            }

            if should_stop_being_crippled {
                beartrap_affected.crippled_data = None;
            }
        }
    }
}
