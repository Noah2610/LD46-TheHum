use super::system_prelude::*;

#[derive(Default)]
pub struct UpdatePlayerAnimationSystem;

impl<'a> System<'a> for UpdatePlayerAnimationSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        ReadStorage<'a, BeartrapAffected>,
        ReadStorage<'a, LadderClimber>,
    );

    fn run(
        &mut self,
        (
            settings,
            player_store,
            velocity_store,
            mut transform_store,
            mut animations_container_store,
            beartrap_affected_store,
            ladder_climber_store,
        ): Self::SystemData,
    ) {
        for (
            player,
            velocity,
            transform,
            animations,
            beartrap_affected,
            ladder_climber,
        ) in (
            &player_store,
            &velocity_store,
            &mut transform_store,
            &mut animations_container_store,
            &beartrap_affected_store,
            &ladder_climber_store,
        )
            .join()
        {
            let vel_padding =
                settings.general.player_animation_update_velocity_padding;

            if velocity.x > vel_padding {
                let scale = transform.scale_mut();
                scale.x = scale.x.abs() * -1.0;
            } else if velocity.x < -vel_padding {
                let scale = transform.scale_mut();
                scale.x = scale.x.abs();
            }

            if !ladder_climber.is_climbing {
                // NOT CLIMBING
                if player.on_ground {
                    if velocity.x.abs()
                        < settings
                            .general
                            .player_animation_update_velocity_padding
                    {
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
            } else {
                // IS CLIMBING
                if velocity.x.abs() < vel_padding
                    && velocity.y.abs()
                        < settings
                            .general
                            .player_animation_update_velocity_padding
                {
                    let _ = animations.play(AnimationKey::ClimbingIdle);
                } else {
                    let _ = animations.play(AnimationKey::Climbing);
                }
            }
        }
    }
}
