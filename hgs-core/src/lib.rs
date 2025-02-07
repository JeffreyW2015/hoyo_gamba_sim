use rand::Rng;

const FIVE_STAR_BASE_RATE: f64 = 0.007; // 0.7%
const FOUR_STAR_BASE_RATE: f64 = 0.05; // 5%

#[derive(Debug, PartialEq)]
pub enum Rarity {
    ThreeStar,
    FourStar,
    FiveStar,
}

pub fn simulate_pull<R: Rng>(rng: &mut R) -> Rarity {
    let roll: f64 = rng.random();

    if roll < FIVE_STAR_BASE_RATE {
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
        let mut rng = StepRng::new(0, 0);
        let rarity = simulate_pull(&mut rng);
        assert_eq!(rarity, Rarity::FiveStar)
    }

    #[test]
    fn test_simulate_pull_four_star() {
        let initial = ((FIVE_STAR_BASE_RATE + FOUR_STAR_BASE_RATE) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0);
        let rarity = simulate_pull(&mut rng);
        assert_eq!(rarity, Rarity::FourStar)
    }

    #[test]
    fn test_simulate_pull_three_star() {
        let initial = ((FIVE_STAR_BASE_RATE + FOUR_STAR_BASE_RATE + 1f64) * u64::MAX as f64) as u64;
        let mut rng = StepRng::new(initial, 0,
        );
        let rarity = simulate_pull(&mut rng);
        assert_eq!(rarity, Rarity::ThreeStar)
    }
}
