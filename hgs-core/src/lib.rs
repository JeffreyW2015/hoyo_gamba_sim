use rand::Rng;

const FIVE_STAR_BASE_RATE: f64 = 0.007; // 0.7%
const FIVE_STAR_HARD_PITY: u8 = 90;
const FIVE_STAR_SOFT_PITY_INCREASE: f64 = 0.06; // 6%
const FIVE_STAR_SOFT_PITY_START: u8 = 74;
const FIVE_STAR_LIMITED_RATE: f64 = 0.55; // 50/50 is more like 55/45

const FOUR_STAR_BASE_RATE: f64 = 0.05; // 5%

#[derive(Debug, PartialEq)]
pub enum Rarity {
    ThreeStar,
    FourStar,
    FiveStar(FiveStarType),
}

#[derive(Debug, PartialEq)]
pub enum FiveStarType {
    Standard,
    Limited
}

pub struct GachaState {
    pub guaranteed: bool,
    pub pity: u8,
}

impl GachaState {
    pub fn new() -> Self {
        GachaState {
            guaranteed: false,
            pity: 0,
        }
    }

    pub fn simulate_pull<R: Rng>(&mut self, rng: &mut R) -> Rarity {
        self.pity += 1;
        if self.pity == FIVE_STAR_HARD_PITY {
            return Rarity::FiveStar(self.pull_five_star(rng));
        }

        let five_star_rate = if self.pity >= FIVE_STAR_SOFT_PITY_START {
            let increase = FIVE_STAR_SOFT_PITY_INCREASE * (self.pity - (FIVE_STAR_SOFT_PITY_START - 1)) as f64;
            FIVE_STAR_BASE_RATE + increase
        } else {
            FIVE_STAR_BASE_RATE
        };

        let roll: f64 = rng.random();

        if roll < five_star_rate {
            Rarity::FiveStar(self.pull_five_star(rng))
        } else if roll < (FOUR_STAR_BASE_RATE + FOUR_STAR_BASE_RATE) {
            // rate is independent of lower odds thing occuring
            Rarity::FourStar
        } else {
            Rarity::ThreeStar
        }
    }

    fn pull_five_star<R: Rng>(&mut self, rng: &mut R) -> FiveStarType {
        self.pity = 0;
        if self.guaranteed {
            self.guaranteed = false;
            return FiveStarType::Limited;
        }

        let roll: f64 = rng.random();
        if roll < FIVE_STAR_LIMITED_RATE {
            FiveStarType::Limited
        } else {
            self.guaranteed = true;
            FiveStarType::Standard
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
        let initial = (FIVE_STAR_BASE_RATE * u64::MAX as f64) as u64; // just 'barely', 0 would also work
        let mut state = GachaState::new();
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.simulate_pull(&mut rng);

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.pity, 0);
    }

    #[test]
    fn test_simulate_pull_four_star() {
        let initial = ((FIVE_STAR_BASE_RATE + FOUR_STAR_BASE_RATE) * u64::MAX as f64) as u64;
        let mut state = GachaState::new();
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.simulate_pull(&mut rng);

        assert_eq!(rarity, Rarity::FourStar);
        assert_eq!(state.pity, 1);
    }

    #[test]
    fn test_simulate_pull_three_star() {
        let initial = ((FIVE_STAR_BASE_RATE + FOUR_STAR_BASE_RATE + 1f64) * u64::MAX as f64) as u64;
        let mut state = GachaState::new();
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.simulate_pull(&mut rng);

        assert_eq!(rarity, Rarity::ThreeStar);
        assert_eq!(state.pity, 1);
    }

    #[test]
    fn test_simulate_pull_hard_pity_five_star() {
        let mut rng = StepRng::new(0,0);
        let mut state = GachaState::new();
        state.pity = FIVE_STAR_HARD_PITY - 1;

        let rarity = state.simulate_pull(&mut rng);

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.pity, 0);
    }

    #[test]
    fn test_simulate_pull_soft_pity_five_star() {
        let mut state = GachaState::new();
        state.pity = FIVE_STAR_SOFT_PITY_START + 1u8;
        let pity_increase = (state.pity - (FIVE_STAR_SOFT_PITY_START - 1)) as f64 * FIVE_STAR_SOFT_PITY_INCREASE; // extra minus 1 for goofy off by one errors (e.g. + 79 pulls)

        let initial = ((FIVE_STAR_BASE_RATE + pity_increase) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial,0);

        let rarity = state.simulate_pull(&mut rng);

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.pity, 0);
    }

    #[test]
    fn test_simulate_pull_soft_pity_multiple_rolls() {
        let mut state = GachaState::new();
        state.pity = FIVE_STAR_SOFT_PITY_START - 2u8;
        let pity_increase = FIVE_STAR_SOFT_PITY_INCREASE;

        let initial = ((FIVE_STAR_BASE_RATE + pity_increase) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial,0);

        // first pull
        let rarity = state.simulate_pull(&mut rng); // no soft pity
        assert_ne!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.pity, 73);

        // second pull (now with pity)
        let rarity = state.simulate_pull(&mut rng); // soft pity start
        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.pity, 0);
    }

    #[test]
    fn test_simulate_pull_five_star_standard() {
        let mut state = GachaState::new();
        state.pity = FIVE_STAR_HARD_PITY -1; // guarantee next

        let initial = ((FIVE_STAR_LIMITED_RATE + 0.1) * u64::MAX as f64) as u64; // "lose" roll, hence + 0.1
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.simulate_pull(&mut rng);

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Standard));
        assert_eq!(state.guaranteed, true);
        assert_eq!(state.pity, 0);
    }

    #[test]
    fn test_simulate_pull_five_star_limited() {
        let mut state = GachaState::new();
        state.pity = FIVE_STAR_HARD_PITY -1; // guarantee next

        let initial = ((FIVE_STAR_LIMITED_RATE - 0.1) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        let rarity = state.simulate_pull(&mut rng);

        assert_eq!(rarity, Rarity::FiveStar(FiveStarType::Limited));
        assert_eq!(state.guaranteed, false);
        assert_eq!(state.pity, 0);
    }
}
