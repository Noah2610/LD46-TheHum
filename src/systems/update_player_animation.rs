use super::system_prelude::*;

const VEL_PADDING: f32 = 10.0;

#[derive(Default)]
pub struct UpdatePlayerAnimationSystem;

impl<'a> System<'a> for UpdatePlayerAnimationSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        ReadStorage<'a, BeartrapAffected>,
    );

    fn run(
        &mut self,
        (
            player_store,
            velocity_store,
            mut transform_store,
            mut animations_container_store,
            beartrap_affected_store,
        ): Self::SystemData,
    ) {
        for (player, velocity, transform, animations, beartrap_affected) in (
            &player_store,
            &velocity_store,
            &mut transform_store,
            &mut animations_container_store,
            &beartrap_affected_store,
        )
            .join()
        {
            if velocity.x > VEL_PADDING {
                let scale = transform.scale_mut();
                scale.x = scale.x.abs() * -1.0;
            } else if velocity.x < -VEL_PADDING {
                let scale = transform.scale_mut();
                scale.x = scale.x.abs();
            }

            if player.on_ground {
                if velocity.x.abs() < VEL_PADDING {
                    let _ = if beartrap_affected.is_crippled() {
                        animations.play(AnimationKey::CrippledIdle)
                    } else {
                        animations.play(AnimationKey::Idle)
                    };
                } else {
                    let _ = if beartrap_affected.is_crippled() {
                        animations.play(AnimationKey::CrippledWalk)
                    } else {
                        animations.play(AnimationKey::Walk)
                    };
                }
            } else {
                let _ = animations.play(AnimationKey::Jump);
            }
        }
    }
}
