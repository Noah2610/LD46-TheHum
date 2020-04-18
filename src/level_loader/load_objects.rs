use super::load_prelude::*;

pub(super) fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
) -> amethyst::Result<()> {
    for object in objects {
        let transform: Transform = (&object).into();
    }

    Ok(())
}
