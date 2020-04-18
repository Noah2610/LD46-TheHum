use super::state_prelude::*;
use crate::resource;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        insert_resources(data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(MainMenu::default()))
    }
}

fn insert_resources(world: &mut World) {
    let settings = Settings::load().unwrap();
    world.insert(settings);

    let sprite_sheet_handles = SpriteSheetHandles::<PathBuf>::default();
    world.insert(sprite_sheet_handles);

    load_songs(world);
    load_sounds(world);
}

fn load_songs(world: &mut World) {
    let songs_settings = world.read_resource::<Settings>().songs.clone();

    let mut songs = Songs::<SongKey>::default();

    for (song_key, song_settings) in songs_settings.songs {
        songs
            .load_audio(
                song_key,
                resource(format!("audio/bgm/{}", song_settings.file)),
                song_settings.should_loop,
                world,
            )
            .unwrap();
    }

    world.insert(songs);
}

fn load_sounds(world: &mut World) {
    let sounds_settings = world.read_resource::<Settings>().sounds.clone();

    let mut sounds = Sounds::<SoundKey>::default();

    for (sound_key, sound_settings) in sounds_settings.sounds {
        sounds
            .load_audio(
                sound_key,
                resource(format!("audio/sfx/{}", sound_settings.file)),
                world,
            )
            .unwrap();
    }

    world.insert(sounds);
}
