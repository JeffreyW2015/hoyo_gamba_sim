#[derive(Debug, PartialEq)]
pub enum Rarity {
    ThreeStar,
    FourStar(FourStarType),
    FiveStar(FiveStarType),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FiveStarType {
    Standard,
    Limited
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FourStarType {
    BannerA,
    BannerB,
    BannerC,
    NonBannerCharacter,
    LightCone
}