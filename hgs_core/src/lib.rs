use rand::Rng;

pub mod enums;
pub mod errors;
pub mod settings;

use enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Pull, Rarity};
use settings::{RaritySettings, Settings, SoftPitySettings};

#[derive(Debug, Clone, Copy)]
struct PityState {
    guaranteed: Option<bool>,
    current: u64,
}

impl PityState {
    fn from_rarity_settings(settings: &RaritySettings) -> Self {
        PityState {
            guaranteed: settings.guarantee_enabled.then(|| false),
            current: 0,
        }
    }

    fn increment(&mut self) {
        self.current = self.current.saturating_add(1);
    }

    fn reset_pity(&mut self) {
        self.current = 0;
    }
}

#[derive(Debug)]
pub struct GachaState {
    five_star_state: PityState,
    four_star_state: PityState,

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

    pub fn pull<R: Rng>(&mut self, rng: &mut R) -> Pull {
        self.five_star_state.increment();
        self.four_star_state.increment();

        if self.check_hard_pity(Rarity::FiveStar) {
            return Pull::FiveStar(self.pull_five_star(rng));
        }

        let roll: f64 = rng.random();

        let five_star_rate = self.get_rate(Rarity::FiveStar);

        if roll < five_star_rate {
            Pull::FiveStar(self.pull_five_star(rng))
        } else if self.check_hard_pity(Rarity::FourStar)
            || roll < (five_star_rate + self.get_rate(Rarity::FourStar))
        {
            let four_star = self.pull_four_star(rng);
            Pull::FourStar(four_star)
        } else {
            Pull::ThreeStar
        }
    }

    fn check_hard_pity(&self, rarity: Rarity) -> bool {
        let (pity_state, maybe_hard_pity) = match rarity {
            Rarity::FiveStar => (
                &self.five_star_state,
                &self.settings.five_star_settings.hard_pity,
            ),
            Rarity::FourStar => (
                &self.four_star_state,
                &self.settings.four_star_settings.hard_pity,
            ),
            Rarity::ThreeStar => unreachable!(),
        };

        match maybe_hard_pity {
            Some(hard_pity) => pity_state.current >= *hard_pity,
            None => false,
        }
    }

    fn get_rate(&self, rarity: Rarity) -> f64 {
        let (settings, pity_state) = match rarity {
            Rarity::FiveStar => (&self.settings.five_star_settings, &self.five_star_state),
            Rarity::FourStar => (&self.settings.four_star_settings, &self.four_star_state),
            Rarity::ThreeStar => unreachable!(),
        };

        match settings.soft_pity_settings {
            Some(soft_pity_settings) if pity_state.current >= soft_pity_settings.pity_start => {
                GachaState::soft_pity_rate(
                    pity_state.current,
                    &soft_pity_settings,
                    settings.base_rate,
                )
            }
            _ => settings.base_rate,
        }
    }

    fn soft_pity_rate(current: u64, settings: &SoftPitySettings, base_rate: f64) -> f64 {
        let soft_pity_count = current - (settings.pity_start - 1);
        let increase = settings.rate_increase * soft_pity_count as f64;
        base_rate + increase
    }

    fn pull_five_star<R: Rng>(&mut self, rng: &mut R) -> FiveStarType {
        self.five_star_state.reset_pity();
        let pull = match self.five_star_state.guaranteed {
            Some(true) => FiveStarType::Limited,
            Some(false) | None => self.roll_five_star(rng),
        };

        if let Some(guaranteed) = &mut self.five_star_state.guaranteed {
            *guaranteed = match pull {
                FiveStarType::Limited => false,
                FiveStarType::Standard => true,
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

    fn pull_four_star<R: Rng>(&mut self, rng: &mut R) -> FourStarType {
        self.four_star_state.reset_pity();

        let pull = match &self.four_star_state.guaranteed {
            Some(true) => FourStarType::Banner(self.pull_banner_four_star(rng)),
            Some(false) | None => self.roll_four_star(rng),
        };

        if let Some(guaranteed) = &mut self.four_star_state.guaranteed {
            *guaranteed = match pull {
                FourStarType::Banner(_) => false,
                FourStarType::Loss(_) => true,
            };
        }

        pull
    }

    fn roll_four_star<R: Rng>(&self, rng: &mut R) -> FourStarType {
        let roll = rng.random_range(0..2);
        match roll {
            0 => {
                if let Some(rate) = self.settings.four_star_settings.banner_rate {
                    let roll: f64 = rng.random();
                    if roll < rate {
                        let banner = self.pull_banner_four_star(rng);
                        return FourStarType::Banner(banner);
                    }
                }
                FourStarType::Loss(FourStarLoss::Character)
            }
            1 => FourStarType::Loss(FourStarLoss::LightCone),
            _ => unreachable!(),
        }
    }

    fn pull_banner_four_star<R: Rng>(&self, rng: &mut R) -> FourStarBanner {
        let roll = rng.random_range(0..3);
        match roll {
            0 => FourStarBanner::A,
            1 => FourStarBanner::B,
            2 => FourStarBanner::C,
            _ => unreachable!(),
        }
    }
}