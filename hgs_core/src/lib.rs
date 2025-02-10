pub mod settings;
use settings::Settings;

pub mod enums;
use enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Rarity};

use rand::Rng;

pub struct GachaState {
    previous_five_star: Option<FiveStarType>,
    five_star_pity: u8,

    previous_four_star: Option<FourStarType>,
    four_star_pity: u8,

    settings: Settings,
}

impl GachaState {
    pub fn new(settings: Settings) -> Self {
        GachaState {
            previous_five_star: None,
            five_star_pity: 0,

            previous_four_star: None,
            four_star_pity: 0,

            settings,
        }
    }

    pub fn pull<R: Rng>(&mut self, rng: &mut R) -> Rarity {
        self.five_star_pity += 1;
        self.four_star_pity += 1;
        if self.five_star_pity >= self.settings.five_star_hard_pity {
            return Rarity::FiveStar(self.pull_five_star(rng));
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
            Rarity::FiveStar(self.pull_five_star(rng))
        } else if self.four_star_pity >= self.settings.four_star_hard_pity
            || roll < (five_star_rate + self.settings.four_star_base_rate)
        {
            Rarity::FourStar(self.pull_four_star(rng))
        } else {
            Rarity::ThreeStar
        }
    }

    fn pull_five_star<R: Rng>(&mut self, rng: &mut R) -> FiveStarType {
        self.five_star_pity = 0;
        let pull = match self.previous_five_star {
            None | Some(FiveStarType::Limited) => self.roll_five_star(rng),
            Some(FiveStarType::Standard) => FiveStarType::Limited,
        };

        self.previous_five_star = Some(pull);

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

    fn pull_four_star<R: Rng>(&mut self, rng: &mut R) -> FourStarType {
        self.four_star_pity = 0;
        let pull = match self.previous_four_star {
            None | Some(FourStarType::Loss(_)) => {
                FourStarType::Banner(self.pull_banner_four_star(rng))
            }
            _ => self.roll_four_star(rng),
        };

        self.previous_four_star = Some(pull);

        pull
    }

    fn roll_four_star<R: Rng>(&self, rng: &mut R) -> FourStarType {
        // 50/50 shot for character vs lightcone, if character, 50/50 shot for banner
        let roll = rng.random_range(0..2);
        match roll {
            0 => {
                let roll: f64 = rng.random();
                if roll < self.settings.four_star_banner_rate {
                    FourStarType::Banner(self.pull_banner_four_star(rng))
                } else {
                    FourStarType::Loss(FourStarLoss::Character)
                }
            }
            1 => FourStarType::Loss(FourStarLoss::LightCone),
            _ => panic!("impossible roll four star character vs light cone"),
        }
    }

    fn pull_banner_four_star<R: Rng>(&self, rng: &mut R) -> FourStarBanner {
        let roll = rng.random_range(0..3);
        match roll {
            0 => FourStarBanner::A,
            1 => FourStarBanner::B,
            2 => FourStarBanner::C,
            _ => panic!("impossible roll for four star banner win"),
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

        let rarity = state.pull(&mut rng);

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

        let rarity = state.pull(&mut rng);

        assert_eq!(
            rarity,
            Rarity::FourStar(FourStarType::Banner(FourStarBanner::C))
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

        let rarity = state.pull(&mut rng);

        assert_eq!(rarity, Rarity::ThreeStar);
        assert_eq!(state.five_star_pity, 1);
    }

    #[test]
    fn test_simulate_pull_hard_pity_five_star() {
        let settings = Settings::default();
        let mut rng = StepRng::new(0, 0);
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_hard_pity - 1;

        let rarity = state.pull(&mut rng);

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

        let rarity = state.pull(&mut rng);

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
        let rarity = state.pull(&mut rng); // no soft pity
        assert_ne!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 73);

        // second pull (now with pity)
        let rarity = state.pull(&mut rng); // soft pity start
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

        let rarity = state.pull(&mut rng);

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Standard));
        assert_eq!(state.previous_five_star, Some(FiveStarType::Standard));
        assert_eq!(state.five_star_pity, 0);
    }

    #[test]
    fn test_simulate_pull_five_star_limited() {
        let settings = Settings::default();
        let mut state = GachaState::new(settings);
        state.five_star_pity = state.settings.five_star_hard_pity - 1; // guarantee next

        let initial = ((state.settings.five_star_limited_rate - 0.1) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.pull(&mut rng);

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.previous_five_star, Some(FiveStarType::Limited));
        assert_eq!(state.five_star_pity, 0);
    }
}
