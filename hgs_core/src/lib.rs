pub mod settings;
use settings::Settings;

pub mod enums;
use enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Rarity};

pub mod errors;
use errors::PullError;

use rand::Rng;

pub struct GachaState {
    five_star_guaranteed: bool,
    five_star_pity: u8,

    four_star_guaranteed: bool,
    four_star_pity: u8,

    settings: Settings,
}

impl GachaState {
    pub fn new(settings: Settings) -> Self {
        GachaState {
            five_star_guaranteed: false,
            five_star_pity: 0,

            four_star_guaranteed: false,
            four_star_pity: 0,

            settings,
        }
    }

    pub fn pull<R: Rng>(&mut self, rng: &mut R) -> Result<Rarity, PullError> {
        self.five_star_pity += 1;
        self.four_star_pity += 1;
        if self.five_star_pity >= self.settings.five_star_hard_pity {
            return Ok(Rarity::FiveStar(self.pull_five_star(rng)));
        }

        let five_star_rate = if self.five_star_pity >= self.settings.five_star_soft_pity {
            let soft_pity_count = self.five_star_pity - (self.settings.five_star_soft_pity - 1);
            let increase = self.settings.five_star_soft_pity_rate_increase * soft_pity_count as f64;
            self.settings.five_star_base_rate + increase
        } else {
            self.settings.five_star_base_rate
        };

        let roll: f64 = rng.random();

        if roll < five_star_rate {
            Ok(Rarity::FiveStar(self.pull_five_star(rng)))
        } else if self.four_star_pity >= self.settings.four_star_hard_pity
            || roll < (five_star_rate + self.settings.four_star_base_rate)
        {
            let four_star = self.pull_four_star(rng)?;
            Ok(Rarity::FourStar(four_star))
        } else {
            Ok(Rarity::ThreeStar)
        }
    }

    fn pull_five_star<R: Rng>(&mut self, rng: &mut R) -> FiveStarType {
        self.five_star_pity = 0;
        let pull = if self.five_star_guaranteed {
            FiveStarType::Limited
        } else {
            self.roll_five_star(rng)
        };

        self.five_star_guaranteed = match pull {
            FiveStarType::Limited => false,
            FiveStarType::Standard => true
        };

        pull
    }

    fn roll_five_star<R: Rng>(&self, rng: &mut R) -> FiveStarType {
        let roll: f64 = rng.random();

        if roll < self.settings.five_star_limited_rate {
            FiveStarType::Limited
        } else {
            FiveStarType::Standard
        }
    }

    fn pull_four_star<R: Rng>(&mut self, rng: &mut R) -> Result<FourStarType, PullError> {
        self.four_star_pity = 0;
        let pull = if self.four_star_guaranteed {
            let banner = self.pull_banner_four_star(rng)?;
            FourStarType::Banner(banner)
        } else {
            self.roll_four_star(rng)?
        };

        self.four_star_guaranteed = match pull {
            FourStarType::Banner(_) => false,
            FourStarType::Loss(_) => true,
        };

        Ok(pull)
    }

    fn roll_four_star<R: Rng>(&self, rng: &mut R) -> Result<FourStarType, PullError> {
        // 50/50 shot for character vs lightcone, if character, 50/50 shot for banner
        let roll = rng.random_range(0..2);
        match roll {
            0 => {
                let roll: f64 = rng.random();
                if roll < self.settings.four_star_banner_rate {
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
        let settings = Settings::default();
        let initial = (settings.five_star_base_rate * u64::MAX as f64) as u64; // just 'barely', 0 would also work
        let mut state = GachaState::new(settings);
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.pull(&mut rng).unwrap();

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
        assert_eq!(state.four_star_pity, 1);
    }

    #[test]
    fn test_simulate_pull_four_star() {
        let settings = Settings::default();
        let initial = ((settings.five_star_base_rate + settings.four_star_base_rate)
            * u64::MAX as f64) as u64;
        let mut state = GachaState::new(settings);
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.pull(&mut rng).unwrap();

        assert_eq!(
            rarity,
            Rarity::FourStar(FourStarType::Loss(FourStarLoss::LightCone))
        );
        assert_eq!(state.five_star_pity, 1);
        assert_eq!(state.four_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_three_star() {
        let settings = Settings::default();
        let initial = ((settings.five_star_base_rate + settings.four_star_base_rate + 1f64)
            * u64::MAX as f64) as u64;
        let mut state = GachaState::new(settings);
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.pull(&mut rng).unwrap();

        assert_eq!(rarity, Rarity::ThreeStar);
        assert_eq!(state.five_star_pity, 1);
    }

    #[test]
    fn test_simulate_pull_hard_pity_five_star() {
        let settings = Settings::default();
        let mut rng = StepRng::new(0, 0);
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_hard_pity - 1;

        let rarity = state.pull(&mut rng).unwrap();

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_soft_pity_five_star() {
        let settings = Settings::default();
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_soft_pity + 1u8;
        let pity_increase = (state.five_star_pity - (state.settings.five_star_soft_pity - 1))
            as f64
            * state.settings.five_star_soft_pity_rate_increase; // extra minus 1 for goofy off by one errors (e.g. + 79 pulls)

        let initial =
            ((state.settings.five_star_base_rate + pity_increase) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.pull(&mut rng).unwrap();

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_soft_pity_multiple_rolls() {
        let settings = Settings::default();
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_soft_pity - 2u8;
        let pity_increase = state.settings.five_star_soft_pity_rate_increase;

        let initial =
            ((state.settings.five_star_base_rate + pity_increase) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        // first pull
        let rarity = state.pull(&mut rng).unwrap(); // no soft pity
        assert_ne!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 73);

        // second pull (now with pity)
        let rarity = state.pull(&mut rng).unwrap(); // soft pity start
        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_five_star_standard() {
        let settings = Settings::default();
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_hard_pity - 1; // guarantee next

        let initial = ((state.settings.five_star_limited_rate + 0.1) * u64::MAX as f64) as u64; // "lose" roll, hence + 0.1
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.pull(&mut rng).unwrap();

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Standard));
        assert_eq!(state.five_star_guaranteed, true);
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_five_star_limited() {
        let settings = Settings::default();
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_hard_pity - 1; // guarantee next

        let initial = ((state.settings.five_star_limited_rate - 0.1) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.pull(&mut rng).unwrap();

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_guaranteed, false);
        assert_eq!(state.five_star_pity, 0);
    }
}
