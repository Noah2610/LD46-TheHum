use super::system_prelude::*;

#[derive(Default)]
pub struct HandleMovablesSystem;

impl<'a> System<'a> for HandleMovablesSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (time, mut movable_store, mut velocity_store): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (movable, velocity) in
            (&mut movable_store, &mut velocity_store).join()
        {
            let data = movable.data.clone();

            for action in movable.drain_actions() {
                match action {
                    MoveAction::Walk(mult) => {
                        velocity.increase_with_max(
                            &Axis::X,
                            data.acceleration * mult * dt,
                            data.max_velocity,
                        );
                    }

                    MoveAction::Jump => {
                        velocity.increase(&Axis::Y, data.jump_strength);
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
                            data.climb_acceleration * mult * dt,
                            data.max_climb_velocity,
                        );
                    }
                }
            }
        }
    }
}
