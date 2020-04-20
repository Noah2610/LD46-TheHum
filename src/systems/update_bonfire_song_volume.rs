use super::system_prelude::*;

pub struct UpdateBonfireSongVolumeSystem {
    factor:      f32,
    prev_volume: Option<f32>,
}

impl UpdateBonfireSongVolumeSystem {
    pub fn new(factor: f32) -> Self {
        Self {
            factor,
            prev_volume: Default::default(),
        }
    }
}

impl<'a> System<'a> for UpdateBonfireSongVolumeSystem {
    type SystemData = (
        Write<'a, Songs<SongKey>>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Bonfire>,
    );

    fn run(
        &mut self,
        (
            mut songs,
            transform_store,
            player_store,
            bonfire_store,
        ): Self::SystemData,
    ) {
        let player_pos = (&player_store, &transform_store)
            .join()
            .next()
            .map(|(_, transform)| transform.translation());
        let bonfire_pos = (&bonfire_store, &transform_store)
            .join()
            .next()
            .map(|(_, transform)| transform.translation());

        if let (Some(player_pos), Some(bonfire_pos)) = (player_pos, bonfire_pos)
        {
            // https://stackoverflow.com/a/20916980/10927893
            let dist = {
                let x = player_pos.x - bonfire_pos.x;
                let y = player_pos.y - bonfire_pos.y;
                (x * x + y * y).sqrt()
            };

            let new_volume = (1.0 - dist * self.factor).max(0.0);
            let prev_volume = self.prev_volume.get_or_insert(1.0);

            if *prev_volume != new_volume {
                songs
                    .get_mut(&SongKey::Bonfire)
                    .map(|song| song.set_volume(new_volume));
                *prev_volume = new_volume;
            }
        }
    }
}
