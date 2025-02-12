use rand::Rng;

pub mod enums;
pub mod errors;
pub mod settings;

use enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Pull, Rarity};
use errors::PullError;
use settings::{RaritySettings, Settings, SoftPitySettings};

#[derive(Debug)]
pub struct PityState {
    guaranteed: Option<bool>,
    current: u64,
}

impl PityState {
    pub fn from_rarity_settings(settings: &RaritySettings) -> Option<Self>{
        settings.use_pity_state().then(|| PityState {
            guaranteed: settings.guarantee_enabled.then(|| false),
            current: 0,
        })
    }

    pub fn increment(&mut self) {
        self.current = self.current.saturating_add(1);
    }

    pub fn reset_pity(&mut self) {
        self.current = 0;
    }
}

#[derive(Debug)]
pub struct GachaState {
    five_star_state: Option<PityState>,
    four_star_state: Option<PityState>,

    settings: Settings,
}

impl GachaState {
    pub fn new(settings: Settings) -> Self {
        GachaState {
            five_star_state: PityState::from_rarity_settings(&settings.five_star_settings),
            four_star_state: PityState::from_rarity_settings(&settings.four_star_settings),

            settings,
        }
    }

    pub fn pull<R: Rng>(&mut self, rng: &mut R) -> Result<Pull, PullError> {
        if let Some(ref mut pity_state) = self.five_star_state {
            pity_state.increment();
        }
        if let Some(ref mut pity_state) = self.four_star_state {
            pity_state.increment();
        }

        if self.check_hard_pity(Rarity::FiveStar)? {
            return Ok(Pull::FiveStar(self.pull_five_star(rng)));
        }

        let roll: f64 = rng.random();

        let five_star_rate = self.get_rate(Rarity::FiveStar)?;

        if roll < five_star_rate {
            Ok(Pull::FiveStar(self.pull_five_star(rng)))
        } else if self.check_hard_pity(Rarity::FourStar)?
            || roll < (five_star_rate + self.get_rate(Rarity::FourStar)?)
        {
            let four_star = self.pull_four_star(rng)?;
            Ok(Pull::FourStar(four_star))
        } else {
            Ok(Pull::ThreeStar)
        }
    }

    fn check_hard_pity(&self, rarity: Rarity) -> Result<bool, PullError> {
        let determine_result = |pity_state_option: &Option<PityState>, settings: &RaritySettings| -> Result<bool, PullError>{
            match(pity_state_option, settings.hard_pity) {
                (Some(pity_state), Some(hard_pity)) => Ok(pity_state.current >= hard_pity),
                (_, None) => Ok(false),
                (None, Some(_)) => Err(PullError::InvalidPitySettings),
            }
        };
        match rarity {
            Rarity::FiveStar => determine_result(&self.five_star_state, &self.settings.five_star_settings),
            Rarity::FourStar => determine_result(&self.four_star_state, &self.settings.four_star_settings),
            Rarity::ThreeStar => Err(PullError::InvalidRarity(Rarity::ThreeStar))
        }
    }

    fn get_rate(&self, rarity: Rarity) -> Result<f64, PullError> {
        let (settings, state_option) = match rarity {
            Rarity::FiveStar => (&self.settings.five_star_settings, &self.five_star_state),
            Rarity::FourStar => (&self.settings.four_star_settings, &self.four_star_state),
            Rarity::ThreeStar => return Err(PullError::InvalidRarity(Rarity::ThreeStar)),
        };

        match (settings.soft_pity_settings, state_option) {
            (Some(soft_pity_settings), Some(pity_state)) if pity_state.current >= soft_pity_settings.pity_start => {
                Ok(GachaState::soft_pity_rate(pity_state.current, &soft_pity_settings, settings.base_rate))
            },
            (Some(_), None) => Err(PullError::InvalidPitySettings),
            _ => Ok(self.settings.five_star_settings.base_rate),
        }
    }

    fn soft_pity_rate(current: u64, settings: &SoftPitySettings, base_rate: f64) -> f64 {
        let soft_pity_count = current - (settings.pity_start - 1);
        let increase = settings.rate_increase * soft_pity_count as f64;
        base_rate + increase
    }

    fn pull_five_star<R: Rng>(&mut self, rng: &mut R) -> FiveStarType {
        if let Some(ref mut pity_state) = self.five_star_state {
            pity_state.reset_pity();
        }
        let pull = match &self.five_star_state {
            Some(pity_state) => match pity_state.guaranteed {
                Some(true) => FiveStarType::Limited,
                Some(false) | None => self.roll_five_star(rng),
            },
            None => self.roll_five_star(rng),
        };

        if let Some(pity_state) = &mut self.five_star_state {
            pity_state.guaranteed = match pull {
                FiveStarType::Limited => Some(false),
                FiveStarType::Standard => Some(true),
            };
        }

        pull
    }

    fn roll_five_star<R: Rng>(&self, rng: &mut R) -> FiveStarType {
        match self.settings.five_star_settings.banner_rate {
            Some(rate) => {
                let roll: f64 = rng.random();
                if roll < rate {
                    FiveStarType::Limited
                } else {
                    FiveStarType::Standard
                }
            }
            None => FiveStarType::Standard,
        }
    }

    fn pull_four_star<R: Rng>(&mut self, rng: &mut R) -> Result<FourStarType, PullError> {
        if let Some(ref mut pity_state) = self.four_star_state {
            pity_state.reset_pity();
        }

        let pull = match &self.four_star_state {
            Some(pity_state) => match pity_state.guaranteed {
                Some(true) => FourStarType::Banner(self.pull_banner_four_star(rng)?),
                Some(false) | None => self.roll_four_star(rng)?,
            },
            None => self.roll_four_star(rng)?,
        };

        if let Some(pity_state) = &mut self.four_star_state {
            pity_state.guaranteed = match pull {
                FourStarType::Banner(_) => Some(false),
                FourStarType::Loss(_) => Some(true),
            };
        }

        Ok(pull)
    }

    fn roll_four_star<R: Rng>(&self, rng: &mut R) -> Result<FourStarType, PullError> {
        let roll = rng.random_range(0..2);
        match roll {
            0 => {
                let roll: f64 = rng.random();
                if roll < self.settings.four_star_settings.base_rate {
                    let banner = self.pull_banner_four_star(rng)?;
                    Ok(FourStarType::Banner(banner))
                } else {
                    Ok(FourStarType::Loss(FourStarLoss::Character))
                }
            }
            1 => Ok(FourStarType::Loss(FourStarLoss::LightCone)),
            _ => Err(PullError::ImpossibleRoll),
        }
    }

    fn pull_banner_four_star<R: Rng>(&self, rng: &mut R) -> Result<FourStarBanner, PullError> {
        let roll = rng.random_range(0..3);
        match roll {
            0 => Ok(FourStarBanner::A),
            1 => Ok(FourStarBanner::B),
            2 => Ok(FourStarBanner::C),
            _ => Err(PullError::ImpossibleRoll),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::u64;

//     use super::*;
//     use rand::rngs::mock::StepRng;

//     #[test]
//     fn test_simulate_pull_five_star() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let initial = (settings.five_star_settings.base_rate * u64::MAX as f64) as u64; // just 'barely', 0 would also work
//         let mut state = GachaState::new(settings);
//         let mut rng = StepRng::new(initial, 0);

//         let pull = state.pull(&mut rng).unwrap();

//         assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
//         assert_eq!(state.five_star_pity, 0);
//         assert_eq!(state.four_star_pity, 1);
//     }

//     #[test]
//     fn test_simulate_pull_four_star() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let initial = ((settings.five_star_settings.base_rate
//             + settings.four_star_settings.base_rate)
//             * u64::MAX as f64) as u64;
//         let mut state = GachaState::new(settings);
//         let mut rng = StepRng::new(initial, 0);

//         let pull = state.pull(&mut rng).unwrap();

//         assert_eq!(
//             pull,
//             Pull::FourStar(FourStarType::Loss(FourStarLoss::LightCone))
//         );
//         assert_eq!(state.five_star_pity, 1);
//         assert_eq!(state.four_star_pity, 0);
//     }

//     #[test]
//     fn test_simulate_pull_three_star() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let initial =
//             ((settings.five_star_settings.base_rate + settings.four_star_settings.base_rate + 1f64)
//                 * u64::MAX as f64) as u64;
//         let mut state = GachaState::new(settings);
//         let mut rng = StepRng::new(initial, 0);

//         let pull = state.pull(&mut rng).unwrap();

//         assert_eq!(pull, Pull::ThreeStar);
//         assert_eq!(state.five_star_pity, 1);
//     }

//     #[test]
//     fn test_simulate_pull_hard_pity_five_star() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let mut rng = StepRng::new(0, 0);
//         let mut state = GachaState::new(settings);
//         state.five_star_pity = state.settings.five_star_settings.hard_pity - 1;

//         let pull = state.pull(&mut rng).unwrap();

//         assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
//         assert_eq!(state.five_star_pity, 0);
//     }

//     #[test]
//     fn test_simulate_pull_soft_pity_five_star() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let mut state = GachaState::new(settings);
//         let soft_pity_settings = state
//             .settings
//             .five_star_settings
//             .soft_pity_settings
//             .unwrap();
//         state.five_star_pity = soft_pity_settings.pity_start + 1;
//         let pity_increase = (state.five_star_pity - (soft_pity_settings.pity_start - 1)) as f64
//             * soft_pity_settings.rate_increase; // extra minus 1 for goofy off by one errors (e.g. + 79 pulls)

//         let initial = ((state.settings.five_star_settings.base_rate + pity_increase)
//             * u64::MAX as f64) as u64;
//         let mut rng = StepRng::new(initial, 0);

//         let pull = state.pull(&mut rng).unwrap();

//         assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
//         assert_eq!(state.five_star_pity, 0);
//     }

//     #[test]
//     fn test_simulate_pull_soft_pity_multiple_rolls() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let mut state = GachaState::new(settings);
//         let soft_pity_settings = state
//             .settings
//             .five_star_settings
//             .soft_pity_settings
//             .unwrap();
//         state.five_star_pity = soft_pity_settings.pity_start - 2;
//         let pity_increase = soft_pity_settings.rate_increase;

//         let initial = ((state.settings.five_star_settings.base_rate + pity_increase)
//             * u64::MAX as f64) as u64;
//         let mut rng = StepRng::new(initial, 0);

//         // first pull
//         let pull = state.pull(&mut rng).unwrap(); // no soft pity
//         assert_ne!(pull, Pull::FiveStar(FiveStarType::Limited));
//         assert_eq!(state.five_star_pity, 73);

//         // second pull (now with pity)
//         let pull = state.pull(&mut rng).unwrap(); // soft pity start
//         assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
//         assert_eq!(state.five_star_pity, 0);
//     }

//     #[test]
//     fn test_simulate_pull_five_star_standard() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let mut state = GachaState::new(settings);
//         state.five_star_pity = state.settings.five_star_settings.hard_pity - 1; // guarantee next

//         let initial = ((state.settings.five_star_settings.banner_rate.unwrap() + 0.1)
//             * u64::MAX as f64) as u64; // "lose" roll, hence + 0.1
//         let mut rng = StepRng::new(initial, 0);

//         let pull = state.pull(&mut rng).unwrap();

//         assert_eq!(pull, Pull::FiveStar(FiveStarType::Standard));
//         assert_eq!(state.five_star_guaranteed, Some(true));
//         assert_eq!(state.five_star_pity, 0);
//     }

//     #[test]
//     fn test_simulate_pull_five_star_limited() {
//         let settings = Settings::from_banner(enums::Banner::Character);
//         let mut state = GachaState::new(settings);
//         state.five_star_pity = state.settings.five_star_settings.hard_pity - 1; // guarantee next

//         let initial = ((state.settings.five_star_settings.banner_rate.unwrap() - 0.1)
//             * u64::MAX as f64) as u64;
//         let mut rng = StepRng::new(initial, 0);

//         let pull = state.pull(&mut rng).unwrap();

//         assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
//         assert_eq!(state.five_star_guaranteed, Some(false));
//         assert_eq!(state.five_star_pity, 0);
//     }
// }
