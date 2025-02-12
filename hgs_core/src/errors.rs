use crate::enums::Rarity;

#[derive(Debug)]
pub enum PullError {
    ImpossibleRoll,
    InvalidPitySettings,
    InvalidRarity(Rarity),
}

impl std::fmt::Display for PullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullError::ImpossibleRoll => write!(f, "Impossible roll occured"),
            PullError::InvalidPitySettings => write!(f, "Pity settings not valid with state"),
            PullError::InvalidRarity(rarity) => write!(f, "Invalid Rarity used: {:?}", rarity),
        }
    }
}
