pub mod prelude {
    pub use super::animation_key::AnimationKey;
    pub use super::collision_tag::{CollisionTag, SolidTag};
    pub use super::dispatcher_id::DispatcherId;
    pub use super::song_key::SongKey;
    pub use super::sound_key::SoundKey;
    pub use deathframe::resources::prelude::*;
}

mod animation_key;
mod collision_tag;
mod dispatcher_id;
mod song_key;
mod sound_key;
