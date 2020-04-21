use super::system_prelude::*;
use deathframe::amethyst::ecs::Component;
use std::marker::PhantomData;

pub struct SongVolumeProximitySystem<C>
where
    C: Component,
{
    song_key:    SongKey,
    factor:      f32,
    prev_volume: Option<f32>,
    _c:          PhantomData<C>,
}

impl<C> SongVolumeProximitySystem<C>
where
    C: Component,
{
    pub fn new(song_key: SongKey, factor: f32) -> Self {
        Self {
            song_key,
            factor,
            prev_volume: Default::default(),
            _c: Default::default(),
        }
    }
}

impl<'a, C> System<'a> for SongVolumeProximitySystem<C>
where
    C: Component,
{
    type SystemData = (
        Write<'a, Songs<SongKey>>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, C>,
    );

    fn run(
        &mut self,
        (
            mut songs,
            transform_store,
            player_store,
            target_store,
        ): Self::SystemData,
    ) {
        if let Some(song) = songs.get_mut(&self.song_key) {
            let player_pos = (&player_store, &transform_store)
                .join()
                .next()
                .map(|(_, transform)| transform.translation());
            let target_pos = (&target_store, &transform_store)
                .join()
                .next()
                .map(|(_, transform)| transform.translation());

            if let (Some(player_pos), Some(target_pos)) =
                (player_pos, target_pos)
            {
                // https://stackoverflow.com/a/20916980/10927893
                let dist = {
                    let x = player_pos.x - target_pos.x;
                    let y = player_pos.y - target_pos.y;
                    (x * x + y * y).sqrt()
                };

                let factor_invert = 1.0 - self.factor;
                let new_volume = (1.0 - dist * factor_invert).max(0.0);
                let prev_volume = self.prev_volume.get_or_insert(1.0);

                if *prev_volume != new_volume {
                    song.set_volume(new_volume);
                    *prev_volume = new_volume;
                }
            }
        }
    }
}
