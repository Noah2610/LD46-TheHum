use super::init_prelude::*;

pub fn init_beartrap(
    world: &mut World,
    transform: Transform,
    spawn_at_woods_amount: usize,
) -> Entity {
    let beartrap_settings = world.read_resource::<Settings>().beartrap.clone();

    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/beartrap.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    let animations = {
        let mut anims = beartrap_settings.animations;
        if let Err(e) = anims.play(AnimationKey::Idle) {
            eprintln!(
                "[WARNING]\n[entities::init_beartrap]\n    Beartrap has no \
                 `Idle` animation configured.\n    {}",
                e
            );
        }
        anims
    };

    world
        .create_entity()
        .with(transform)
        .with(
            beartrap_settings
                .beartrap
                .with_spawn_at_woods_amount(spawn_at_woods_amount),
        )
        .with(beartrap_settings.size)
        .with(beartrap_settings.hitbox)
        .with(animations)
        .with(sprite_render)
        .with(Transparent)
        .with(ScaleOnce::default())
        .with(Collidable::new(CollisionTag::Beartrap))
        // .with(VisibleInFlame::default())
        .with(Hidden)
        .build()
}
