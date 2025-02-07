use rand::Rng;

const FIVE_STAR_BASE_RATE: f64 = 0.007; // 0.7%
const FIVE_STAR_HARD_PITY: u8 = 90;
const FIVE_STAR_SOFT_PITY_INCREASE: f64 = 0.06; // 6%
const FIVE_STAR_SOFT_PITY_START: u8 = 74;

const FOUR_STAR_BASE_RATE: f64 = 0.05; // 5%

#[derive(Debug, PartialEq)]
pub enum Rarity {
    ThreeStar,
    FourStar,
    FiveStar,
}

pub fn simulate_pull<R: Rng>(rng: &mut R, pull_count: u8) -> Rarity {
    if pull_count == FIVE_STAR_HARD_PITY {
        return Rarity::FiveStar
    }

    let five_star_rate = if pull_count >= FIVE_STAR_SOFT_PITY_START {
        let increase = FIVE_STAR_SOFT_PITY_INCREASE * (pull_count - (FIVE_STAR_SOFT_PITY_START - 1)) as f64;
        FIVE_STAR_BASE_RATE + increase
    } else {
        FIVE_STAR_BASE_RATE
    };

    let roll: f64 = rng.random();

    if roll < five_star_rate {
        Rarity::FiveStar
    } else if roll < (FOUR_STAR_BASE_RATE + FOUR_STAR_BASE_RATE) {
        // rate is independent of lower odds thing occuring
        Rarity::FourStar
    } else {
        Rarity::ThreeStar
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
        let mut rng = StepRng::new(initial, 0);

        let rarity = simulate_pull(&mut rng, 0);

        assert_eq!(rarity, Rarity::FiveStar)
    }

    #[test]
    fn test_simulate_pull_four_star() {
        let initial = ((FIVE_STAR_BASE_RATE + FOUR_STAR_BASE_RATE) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);

        let rarity = simulate_pull(&mut rng, 0);

        assert_eq!(rarity, Rarity::FourStar)
    }

    #[test]
    fn test_simulate_pull_three_star() {
        let initial = ((FIVE_STAR_BASE_RATE + FOUR_STAR_BASE_RATE + 1f64) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0,
        );

        let rarity = simulate_pull(&mut rng, 0);

        assert_eq!(rarity, Rarity::ThreeStar)
    }

    #[test]
    fn test_simulate_pull_hard_pity_five_star() {
        let mut rng = StepRng::new(0,0);

        let rarity = simulate_pull(&mut rng, FIVE_STAR_HARD_PITY);

        assert_eq!(rarity, Rarity::FiveStar)
    }

    #[test]
    fn test_simulate_pull_soft_pity_five_star() {
        let pity = FIVE_STAR_SOFT_PITY_START + 15u8;
        let pity_increase = (pity - (FIVE_STAR_SOFT_PITY_START - 1)-1) as f64 * FIVE_STAR_SOFT_PITY_INCREASE;
        let initial = ((FIVE_STAR_BASE_RATE + pity_increase) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial,0);

        let rarity = simulate_pull(&mut rng, pity);

        assert_eq!(rarity, Rarity::FiveStar)
    }
}
