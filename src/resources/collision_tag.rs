use deathframe::physics::CollisionTag as CTag;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum CollisionTag {
    Player,
    Solid,
    Bonfire,
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (CollisionTag::Player, CollisionTag::Solid) => true,
            (CollisionTag::Player, CollisionTag::Bonfire) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum SolidTag {
    Player,
    Solid,
}

impl CTag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::Player, SolidTag::Solid) => true,
            _ => false,
        }
    }
}
