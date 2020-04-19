use super::system_prelude::*;
use deathframe::physics::query::prelude::*;

#[derive(Default)]
pub struct HandleLadderClimbingSystem;

impl<'a> System<'a> for HandleLadderClimbingSystem {
    type SystemData = (
        ReadStorage<'a, Ladder>,
        WriteStorage<'a, LadderClimber>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, BaseFriction>,
    );

    fn run(
        &mut self,
        (
            ladder_store,
            mut ladder_climber_store,
            input_manager,
            collider_store,
            mut movable_store,
            mut gravity_store,
            mut base_friction_store,
        ): Self::SystemData,
    ) {
        for (ladder_climber, collider, movable, gravity, friction) in (
            &mut ladder_climber_store,
            &collider_store,
            &mut movable_store,
            &mut gravity_store,
            &mut base_friction_store,
        )
            .join()
        {
            let in_ladder_collision = {
                use deathframe::physics::query::exp::prelude_variants::*;

                collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&And(vec![IsTag(CollisionTag::Ladder), IsSide(Inner)]))
                    .run()
                    .is_some()
            };

            if ladder_climber.is_climbing {
                if in_ladder_collision {
                    if let Some(climb_y) =
                        input_manager.axis_value(IngameAxis::ClimbLadder)
                    {
                        if climb_y != 0.0 {
                            movable.add_action(MoveAction::Climb(climb_y));
                        }
                    }
                } else {
                    ladder_climber.is_climbing = false;
                    gravity.set_enabled(&Axis::Y, true);
                    friction.set_enabled(&Axis::Y, false);
                }
            } else {
                if let Some(climb_y) =
                    input_manager.axis_value(IngameAxis::ClimbLadder)
                {
                    if climb_y != 0.0 {
                        if in_ladder_collision {
                            ladder_climber.is_climbing = true;
                            gravity.set_enabled(&Axis::Y, false);
                            friction.set_enabled(&Axis::Y, true);
                        }
                    }
                }
            }
        }
    }
}
