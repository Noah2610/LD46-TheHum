use super::init_prelude::*;

pub fn init_wood(world: &mut World, transform: Transform) -> Entity {
    world
        .write_resource::<SpriteSheetHandles<PathBuf>>()
        .load(resource("spritesheets/wood.png"), world);

    world.exec(|(entities, mut storages): (Entities, InitWoodStorages)| {
        init_wood_with_storages(entities.create(), transform, &mut storages)
            .unwrap()
    })
}

use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{
    Entities,
    Read,
    ReadExpect,
    SystemData,
    World,
    WriteStorage,
};

pub fn init_wood_with_storages(
    entity: Entity,
    transform: Transform,
    storages: &mut InitWoodStorages,
) -> amethyst::Result<Entity> {
    let InitWoodStorages {
        settings,
        sprite_sheet_handles,
        transform_store,
        size_store,
        hitbox_store,
        collidable_store,
        sprite_render_store,
        scale_once_store,
        transparent_store,
        visible_in_flame_store,
        hidden_store,
    } = storages;

    let wood_settings = settings.wood.clone();

    transform_store.insert(entity, transform)?;
    size_store.insert(entity, wood_settings.size)?;
    hitbox_store.insert(entity, wood_settings.hitbox)?;
    collidable_store.insert(entity, Collidable::new(CollisionTag::Wood))?;
    if let Some(sprite_sheet) =
        sprite_sheet_handles.get(&resource("spritesheets/wood.png"))
    {
        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        };
        sprite_render_store.insert(entity, sprite_render)?;
    } else {
        eprintln!(
            "[WARNING]\n[entities::wood_spawner::init_wood_with_storages]\n    \
            Wood spritesheet is not loaded"
        );
    }
    scale_once_store.insert(entity, ScaleOnce::default())?;
    transparent_store.insert(entity, Transparent)?;
    visible_in_flame_store.insert(entity, VisibleInFlame::default())?;
    hidden_store.insert(entity, Hidden)?;

    Ok(entity)
}

#[derive(SystemData)]
pub struct InitWoodStorages<'a> {
    pub settings:               ReadExpect<'a, Settings>,
    pub sprite_sheet_handles:   Read<'a, SpriteSheetHandles<PathBuf>>,
    pub transform_store:        WriteStorage<'a, Transform>,
    pub size_store:             WriteStorage<'a, Size>,
    pub hitbox_store:           WriteStorage<'a, Hitbox>,
    pub collidable_store:       WriteStorage<'a, Collidable<CollisionTag>>,
    pub sprite_render_store:    WriteStorage<'a, SpriteRender>,
    pub scale_once_store:       WriteStorage<'a, ScaleOnce>,
    pub transparent_store:      WriteStorage<'a, Transparent>,
    pub visible_in_flame_store: WriteStorage<'a, VisibleInFlame>,
    pub hidden_store:           WriteStorage<'a, Hidden>,
}
