use rand::Rng;

pub mod enums;
pub mod errors;
pub mod settings;

use enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Pull};
use errors::PullError;
use settings::Settings;

#[derive(Debug)]
pub struct GachaState {
    five_star_guaranteed: Option<bool>,
    five_star_pity: u8,

    four_star_guaranteed: Option<bool>,
    four_star_pity: u8,

    settings: Settings,
}

impl GachaState {
    pub fn new(settings: Settings) -> Self {
        GachaState {
            five_star_guaranteed: settings.five_star_settings.guarantee(),
            five_star_pity: 0,

            four_star_guaranteed: settings.four_star_settings.guarantee(),
            four_star_pity: 0,

            settings,
        }
    }

    pub fn pull<R: Rng>(&mut self, rng: &mut R) -> Result<Pull, PullError> {
        self.five_star_pity += 1;
        self.four_star_pity += 1;
        if self.five_star_pity >= self.settings.five_star_settings.hard_pity {
            return Ok(Pull::FiveStar(self.pull_five_star(rng)));
        }

        let five_star_rate = match &self.settings.five_star_settings.soft_pity_settings {
            Some(soft_pity_settings) if self.five_star_pity >= soft_pity_settings.pity_start => {
                let soft_pity_count = self.five_star_pity - (soft_pity_settings.pity_start - 1);
                let increase = soft_pity_settings.rate_increase * soft_pity_count as f64;
                self.settings.five_star_settings.base_rate + increase
            }
            _ => self.settings.five_star_settings.base_rate,
        };

        let roll: f64 = rng.random();

        if roll < five_star_rate {
            Ok(Pull::FiveStar(self.pull_five_star(rng)))
        } else if self.four_star_pity >= self.settings.four_star_settings.hard_pity
            || roll < (five_star_rate + self.settings.four_star_settings.base_rate)
        {
            let four_star = self.pull_four_star(rng)?;
            Ok(Pull::FourStar(four_star))
        } else {
            Ok(Pull::ThreeStar)
        }
    }

    fn pull_five_star<R: Rng>(&mut self, rng: &mut R) -> FiveStarType {
        self.five_star_pity = 0;
        let pull = match self.five_star_guaranteed {
            Some(true) => FiveStarType::Limited,
            Some(false) | None => self.roll_five_star(rng),
        };

        if self.five_star_guaranteed.is_some() {
            self.five_star_guaranteed = match pull {
                FiveStarType::Limited => Some(false),
                FiveStarType::Standard => Some(true),
            };
        }

        pull
    }

    fn roll_five_star<R: Rng>(&self, rng: &mut R) -> FiveStarType {
        match self.settings.five_star_settings.limited_rate {
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
        self.four_star_pity = 0;
        let pull = match self.four_star_guaranteed {
            Some(true) => {
                let banner = self.pull_banner_four_star(rng)?;
                FourStarType::Banner(banner)
            }
            Some(false) | None => self.roll_four_star(rng)?,
        };

        if self.four_star_guaranteed.is_some() {
            match pull {
                FourStarType::Banner(_) => false,
                FourStarType::Loss(_) => true,
            };
        }

        Ok(pull)
    }

    fn roll_four_star<R: Rng>(&self, rng: &mut R) -> Result<FourStarType, PullError> {
        // 50/50 shot for character vs lightcone, if character, 50/50 shot for banner
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

#[cfg(test)]
mod tests {
    use std::u64;

    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_simulate_pull_five_star() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let initial = (settings.five_star_settings.base_rate * u64::MAX as f64) as u64; // just 'barely', 0 would also work
        let mut state = GachaState::new(settings);
        let mut rng = StepRng::new(initial, 0);

        let pull = state.pull(&mut rng).unwrap();

        assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
        assert_eq!(state.four_star_pity, 1);
    }

    #[test]
    fn test_simulate_pull_four_star() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let initial = ((settings.five_star_settings.base_rate
            + settings.four_star_settings.base_rate)
            * u64::MAX as f64) as u64;
        let mut state = GachaState::new(settings);
        let mut rng = StepRng::new(initial, 0);

        let pull = state.pull(&mut rng).unwrap();

        assert_eq!(
            pull,
            Pull::FourStar(FourStarType::Loss(FourStarLoss::LightCone))
        );
        assert_eq!(state.five_star_pity, 1);
        assert_eq!(state.four_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_three_star() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let initial =
            ((settings.five_star_settings.base_rate + settings.four_star_settings.base_rate + 1f64)
                * u64::MAX as f64) as u64;
        let mut state = GachaState::new(settings);
        let mut rng = StepRng::new(initial, 0);

        let pull = state.pull(&mut rng).unwrap();

        assert_eq!(pull, Pull::ThreeStar);
        assert_eq!(state.five_star_pity, 1);
    }

    #[test]
    fn test_simulate_pull_hard_pity_five_star() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let mut rng = StepRng::new(0, 0);
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_settings.hard_pity - 1;

        let pull = state.pull(&mut rng).unwrap();

        assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_soft_pity_five_star() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let mut state = GachaState::new(settings);
        let soft_pity_settings = state
            .settings
            .five_star_settings
            .soft_pity_settings
            .unwrap();
        state.five_star_pity = soft_pity_settings.pity_start + 1u8;
        let pity_increase = (state.five_star_pity - (soft_pity_settings.pity_start - 1)) as f64
            * soft_pity_settings.rate_increase; // extra minus 1 for goofy off by one errors (e.g. + 79 pulls)

        let initial = ((state.settings.five_star_settings.base_rate + pity_increase)
            * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        let pull = state.pull(&mut rng).unwrap();

        assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_soft_pity_multiple_rolls() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let mut state = GachaState::new(settings);
        let soft_pity_settings = state
            .settings
            .five_star_settings
            .soft_pity_settings
            .unwrap();
        state.five_star_pity = soft_pity_settings.pity_start - 2u8;
        let pity_increase = soft_pity_settings.rate_increase;

        let initial = ((state.settings.five_star_settings.base_rate + pity_increase)
            * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        // first pull
        let pull = state.pull(&mut rng).unwrap(); // no soft pity
        assert_ne!(pull, Pull::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 73);

        // second pull (now with pity)
        let pull = state.pull(&mut rng).unwrap(); // soft pity start
        assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_five_star_standard() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_settings.hard_pity - 1; // guarantee next

        let initial = ((state.settings.five_star_settings.limited_rate.unwrap() + 0.1)
            * u64::MAX as f64) as u64; // "lose" roll, hence + 0.1
        let mut rng = StepRng::new(initial, 0);

        let pull = state.pull(&mut rng).unwrap();

        assert_eq!(pull, Pull::FiveStar(FiveStarType::Standard));
        assert_eq!(state.five_star_guaranteed, Some(true));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_five_star_limited() {
        let settings = Settings::from_banner(enums::Banner::Character);
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_settings.hard_pity - 1; // guarantee next

        let initial = ((state.settings.five_star_settings.limited_rate.unwrap() - 0.1)
            * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        let pull = state.pull(&mut rng).unwrap();

        assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_guaranteed, Some(false));
        assert_eq!(state.five_star_pity, 0);
    }
}
