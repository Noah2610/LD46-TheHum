use super::init_prelude::*;

pub fn init_ladder(
    world: &mut World,
    transform: Transform,
    size: Size,
) -> Entity {
    let hitbox = Hitbox::from(vec![(&size).into()]);

    world
        .create_entity()
        .with(transform)
        .with(size)
        .with(hitbox)
        .with(Collidable::new(CollisionTag::Ladder))
        .with(Ladder::default())
        .with(Loadable::default())
        .build()
}
