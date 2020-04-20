use super::init_prelude::*;
use deathframe::core::geo::prelude::{Axis, Rect};

pub fn init_player(
    world: &mut World,
    transform: Transform,
    level_rect: Rect,
) -> Entity {
    let player_settings = world.read_resource::<Settings>().player.clone();

    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/player.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    world
        .create_entity()
        .with(Player::default())
        .with(transform)
        .with(sprite_render)
        .with(Velocity::default())
        .with(ScaleOnce::default())
        .with(Lifecycle::default())
        .with(Transparent)
        .with(Collider::new(CollisionTag::Player))
        .with(Collidable::new(CollisionTag::Player))
        .with(Solid::new(SolidTag::Player))
        .with(TriggerReactiveAnimation::default())
        .with(LadderClimber::default())
        .with(BeartrapAffected::default())
        .with(Confined::from(level_rect))
        .with(player_settings.size)
        .with(player_settings.hitbox)
        .with(player_settings.movable)
        .with({
            let mut fric = player_settings.base_friction;
            fric.set_enabled(&Axis::Y, false);
            fric
        })
        .with(player_settings.gravity)
        .with(player_settings.flame)
        .with(player_settings.wood_inventory)
        .with(player_settings.animations)
        .build()
}
