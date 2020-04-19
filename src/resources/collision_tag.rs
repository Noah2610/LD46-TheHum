use deathframe::physics::CollisionTag as CTag;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum CollisionTag {
    Player,
    Solid,
    Bonfire,
    ReactiveTile,
    Wood,
    Ladder,
    Beartrap,
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (CollisionTag::Player, CollisionTag::Solid) => true,
            (CollisionTag::Player, CollisionTag::Bonfire) => true,
            (CollisionTag::Player, CollisionTag::ReactiveTile) => true,
            (CollisionTag::Player, CollisionTag::Wood) => true,
            (CollisionTag::Player, CollisionTag::Ladder) => true,
            (CollisionTag::Player, CollisionTag::Beartrap) => true,
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
