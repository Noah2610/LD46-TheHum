use super::system_prelude::*;

#[derive(Default)]
pub struct HandleMovablesSystem;

impl<'a> System<'a> for HandleMovablesSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, WoodInventory>,
        ReadStorage<'a, BeartrapAffected>,
    );

    fn run(
        &mut self,
        (
            time,
            mut movable_store,
            mut velocity_store,
            wood_inventory_store,
            beartrap_affected_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (movable, velocity, inventory_opt, beartrap_affected_opt) in (
            &mut movable_store,
            &mut velocity_store,
            wood_inventory_store.maybe(),
            beartrap_affected_store.maybe(),
        )
            .join()
        {
            let data = movable.data.clone();
            let value_with_wood_decrease = |base_acceleration: f32| -> f32 {
                inventory_opt
                    .map(|inventory| {
                        base_acceleration
                            * (1.0
                                + (inventory.woods as f32
                                    / -data.wood_speed_decrease_factor))
                    })
                    .unwrap_or(data.acceleration)
            };

            for action in movable.drain_actions() {
                match action {
                    MoveAction::Walk(mult) => {
                        velocity.increase_with_max(
                            &Axis::X,
                            value_with_wood_decrease(data.acceleration)
                                * mult
                                * dt,
                            value_with_wood_decrease(data.max_velocity),
                        );
                    }

                    MoveAction::Jump => {
                        if inventory_opt
                            .map(|inventory| !inventory.is_at_max())
                            .unwrap_or(true)
                        {
                            if beartrap_affected_opt
                                .map(|beartrap_affected| {
                                    beartrap_affected.crippled_data.is_none()
                                })
                                .unwrap_or(true)
                            {
                                velocity.increase(
                                    &Axis::Y,
                                    value_with_wood_decrease(
                                        data.jump_strength,
                                    ),
                                );
                            }
                        }
                    }

                    MoveAction::KillJump => {
                        let vel = velocity.y;
                        if vel > data.kill_jump_min_velocity {
                            let decreased = (vel + data.kill_jump_strength)
                                .max(data.kill_jump_min_velocity);
                            velocity.set(&Axis::Y, decreased);
                        }
                    }

                    MoveAction::Climb(mult) => {
                        velocity.increase_with_max(
                            &Axis::Y,
                            value_with_wood_decrease(data.climb_acceleration)
                                * mult
                                * dt,
                            value_with_wood_decrease(data.max_climb_velocity),
                        );
                    }
                }
            }
        }
    }
}
