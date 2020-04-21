use super::system_prelude::*;
use climer::Timer;
use std::time::Duration;

#[derive(Default)]
pub struct PlayRandomSoundsSystem {
    timer: Option<Timer>,
}

impl<'a> System<'a> for PlayRandomSoundsSystem {
    type SystemData =
        (ReadExpect<'a, Settings>, Write<'a, SoundPlayer<SoundKey>>);

    fn run(&mut self, (settings, mut sound_player): Self::SystemData) {
        let timer = self.timer.get_or_insert_with(|| {
            Timer::new(
                Some(
                    Duration::from_millis(settings.sounds.update_interval_ms)
                        .into(),
                ),
                None,
            )
        });

        if timer.state.is_stopped() || timer.state.is_finished() {
            timer.start().unwrap();
        }
        timer.update().unwrap();

        if timer.state.is_finished() {
            if let Some(random_sound_key) = settings.sounds.random_sound_key() {
                sound_player.add_action(SoundAction::Play(random_sound_key));
            }
        }
    }
}
