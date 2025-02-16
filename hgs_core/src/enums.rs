#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Banner {
    Standard,
    Character,
    LightCone,
    Departure,
}

#[derive(serde::Deserialize, Debug)]
pub enum Pull {
    Character(u64),
    LightCone(u64),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rarity {
    ThreeStar,
    FourStar,
    FiveStar,
}