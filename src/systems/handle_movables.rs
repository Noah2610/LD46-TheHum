use super::system_prelude::*;

#[derive(Default)]
pub struct HandleMovablesSystem;

impl<'a> System<'a> for HandleMovablesSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, WoodInventory>,
    );

    fn run(
        &mut self,
        (
            time,
            mut movable_store,
            mut velocity_store,
            wood_inventory_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (movable, velocity, inventory_opt) in (
            &mut movable_store,
            &mut velocity_store,
            wood_inventory_store.maybe(),
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
                        velocity.increase(
                            &Axis::Y,
                            value_with_wood_decrease(data.jump_strength),
                        );
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
