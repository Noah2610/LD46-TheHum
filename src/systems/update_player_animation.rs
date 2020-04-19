use super::system_prelude::*;

const IDLE_VEL_PADDING: f32 = 10.0;

#[derive(Default)]
pub struct UpdatePlayerAnimationSystem;

impl<'a> System<'a> for UpdatePlayerAnimationSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    );

    fn run(
        &mut self,
        (
            player_store,
            velocity_store,
            mut transform_store,
            mut animations_container_store,
        ): Self::SystemData,
    ) {
        for (player, velocity, transform, animations) in (
            &player_store,
            &velocity_store,
            &mut transform_store,
            &mut animations_container_store,
        )
            .join()
        {
            if velocity.x.is_sign_negative() {
                let scale = transform.scale_mut();
                scale.x = scale.x.abs();
            } else {
                let scale = transform.scale_mut();
                scale.x = scale.x.abs() * -1.0;
            }

            if player.on_ground {
                if velocity.x.abs() < IDLE_VEL_PADDING {
                    let _ = animations.play(AnimationKey::Idle);
                } else {
                    let _ = animations.play(AnimationKey::Walk);
                }
            } else {
                let _ = animations.play(AnimationKey::Jump);
            }
        }
    }
}
