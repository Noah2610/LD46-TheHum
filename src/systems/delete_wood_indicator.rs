use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteWoodIndicatorSystem;

impl<'a> System<'a> for DeleteWoodIndicatorSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, WoodIndicator>,
        ReadStorage<'a, Animation>,
    );

    fn run(
        &mut self,
        (entities, wood_indicator_store, animation_store): Self::SystemData,
    ) {
        for (entity, _, animation) in
            (&entities, &wood_indicator_store, &animation_store).join()
        {
            if animation.has_played_and_is_finished() {
                let _ = entities.delete(entity);
            }
        }
    }
}
