#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Banner {
    Standard,
    Character,
    LightCone,
    Departure,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pull {
    ThreeStar,
    FourStar(FourStarType),
    FiveStar(FiveStarType),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rarity {
    ThreeStar,
    FourStar,
    FiveStar,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum FiveStarType {
    Standard,
    Limited,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum FourStarType {
    Banner(FourStarBanner),
    Loss(FourStarLoss),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum FourStarBanner {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum FourStarLoss {
    Character,
    LightCone,
}