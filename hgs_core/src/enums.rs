#[derive(Debug, PartialEq)]
pub enum Rarity {
    ThreeStar,
    FourStar,
    FiveStar(FiveStarType),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FiveStarType {
    Standard,
    Limited
}