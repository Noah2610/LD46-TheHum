use super::state_prelude::*;
use crate::level_loader;

pub struct Ingame {
    level_name: String,
}

impl Ingame {
    pub fn new(level_name: String) -> Self {
        Self { level_name }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        level_loader::load_level(
            resource(format!("levels/{}", &self.level_name)),
            data.world,
        )
        .unwrap();

        {
            let mut songs = data.world.write_resource::<Songs<SongKey>>();
            songs.play(&SongKey::Ambience);
            songs.play(&SongKey::Bonfire);
            songs.play(&SongKey::Radio);
        }

        let wood_spawner_manager = data
            .world
            .read_resource::<Settings>()
            .wood_spawner
            .wood_spawner_manager
            .clone();
        data.world.insert(wood_spawner_manager);
    }

    fn on_stop(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        let mut songs = data.world.write_resource::<Songs<SongKey>>();
        songs.stop(&SongKey::Ambience);
        songs.stop(&SongKey::Bonfire);
        songs.stop(&SongKey::Radio);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        use crate::components::prelude::{Bonfire, Flame};
        use amethyst::ecs::{Join, ReadStorage};
        use deathframe::amethyst;

        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        {
            let input_manager =
                data.world.read_resource::<InputManager<MenuBindings>>();
            if input_manager.is_down(MenuAction::Back) {
                return Trans::Pop;
            }
        }

        let game_over = data
            .world
            .exec(
                |(bonfire_store, flame_store): (
                    ReadStorage<Bonfire>,
                    ReadStorage<Flame>,
                )| {
                    (&bonfire_store, &flame_store)
                        .join()
                        .next()
                        .map(|(_, flame)| flame.is_at_min_radius())
                },
            )
            .unwrap_or(false);
        if game_over {
            return Trans::Push(Box::new(GameOver::default()));
        }

        Trans::None
    }
}
