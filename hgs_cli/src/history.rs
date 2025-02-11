use std::collections::HashMap;

use hgs_core::enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Rarity};

pub struct PullHistory {
    pulls: Vec<Rarity>,
    five_star_metadata: FiveStarMetadata,
    four_star_metadata: FourStarMetadata,
}

impl PullHistory {
    pub fn new() -> Self {
        PullHistory {
            pulls: Vec::new(),
            five_star_metadata: FiveStarMetadata::new(),
            four_star_metadata: FourStarMetadata::new(),
        }
    }

    pub fn add(&mut self, rarity: Rarity) {
        self.pulls.push(rarity);

        self.five_star_metadata.pity += 1;
        self.four_star_metadata.pity += 1;

        match rarity {
            Rarity::FiveStar(five_star) => {
                self.five_star_metadata.process(five_star);
            }
            Rarity::FourStar(four_star) => {
                self.four_star_metadata.process(four_star);
            }
            _ => {}
        }
    }

    pub fn total_count(&self) -> usize {
        self.pulls.len()
    }

    pub fn five_star_metadata(&self) -> &FiveStarMetadata {
        &self.five_star_metadata
    }

    pub fn four_star_metadata(&self) -> &FourStarMetadata {
        &self.four_star_metadata
    }
}

pub struct FiveStarMetadata {
    pub pity_to_pulls: HashMap<u8, HashMap<FiveStarType, usize>>,
    pub limited_pulls: usize,
    pub standard_pulls: usize,
    pub pity: u8,

    pub next_guaranteed: bool,
    pub wins: usize,
    pub losses: usize,
    pub guaranteeds: usize,
}

impl FiveStarMetadata {
    fn new() -> Self {
        FiveStarMetadata {
            pity_to_pulls: HashMap::new(),
            limited_pulls: 0,
            standard_pulls: 0,
            pity: 0,
            next_guaranteed: false,
            wins: 0,
            losses: 0,
            guaranteeds: 0,
        }
    }

    pub fn total(&self) -> usize {
        self.limited_pulls + self.standard_pulls
    }

    pub fn _pity_of_type(&self, pity: u8, five_star: FiveStarType) -> usize {
        self.pity_to_pulls
            .get(&pity)
            .and_then(|inner| inner.get(&five_star))
            .copied()
            .unwrap_or(0)
    }

    fn process(&mut self, five_star: FiveStarType) {
        let pity_entry = self
            .pity_to_pulls
            .entry(self.pity)
            .or_insert(HashMap::new());

        match five_star {
            FiveStarType::Limited => {
                pity_entry
                    .entry(FiveStarType::Limited)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                self.limited_pulls += 1;
                if self.next_guaranteed {
                    self.guaranteeds += 1;
                } else {
                    self.wins += 1
                }
                self.next_guaranteed = false;
            }
            FiveStarType::Standard => {
                pity_entry
                    .entry(FiveStarType::Standard)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                self.standard_pulls += 1;
                self.losses += 1;
                self.next_guaranteed = true;
            }
        }
        self.pity = 0;
    }
}

pub struct FourStarMetadata {
    pub pity_to_pulls: HashMap<u8, HashMap<FourStarType, usize>>,
    pub a_pulls: usize,
    pub b_pulls: usize,
    pub c_pulls: usize,
    pub character_non_banner_pulls: usize,
    pub light_cone_pulls: usize,
    pub pity: u8,

    pub next_guaranteed: bool,
    pub wins: usize,
    pub losses: usize,
    pub guaranteeds: usize,
}

impl FourStarMetadata {
    fn new() -> Self {
        FourStarMetadata {
            pity_to_pulls: HashMap::new(),
            a_pulls: 0,
            b_pulls: 0,
            c_pulls: 0,
            character_non_banner_pulls: 0,
            light_cone_pulls: 0,
            pity: 0,
            next_guaranteed: false,
            wins: 0,
            losses: 0,
            guaranteeds: 0,
        }
    }

    pub fn banners(&self) -> usize {
        self.a_pulls + self.b_pulls + self.c_pulls
    }

    pub fn non_banners(&self) -> usize {
        self.character_non_banner_pulls + self.light_cone_pulls
    }

    pub fn total(&self) -> usize {
        self.banners() + self.non_banners()
    }

    pub fn _pity_of_type(&self, pity: u8, four_star: FourStarType) -> usize {
        self.pity_to_pulls
            .get(&pity)
            .and_then(|inner| inner.get(&four_star))
            .copied()
            .unwrap_or(0)
    }

    fn process(&mut self, four_star: FourStarType) {
        let pity_entry = self
            .pity_to_pulls
            .entry(self.pity)
            .or_insert(HashMap::new());

        match four_star {
            FourStarType::Banner(banner) => {
                if self.next_guaranteed {
                    self.guaranteeds += 1;
                } else {
                    self.wins += 1;
                }
                match banner {
                    FourStarBanner::A => {
                        self.a_pulls += 1;
                        pity_entry
                            .entry(FourStarType::Banner(FourStarBanner::A))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    FourStarBanner::B => {
                        self.b_pulls += 1;
                        pity_entry
                            .entry(FourStarType::Banner(FourStarBanner::B))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    FourStarBanner::C => {
                        self.c_pulls += 1;
                        pity_entry
                            .entry(FourStarType::Banner(FourStarBanner::C))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
            FourStarType::Loss(loss) => {
                self.losses += 1;
                match loss {
                    FourStarLoss::Character => {
                        self.character_non_banner_pulls += 1;
                        pity_entry
                            .entry(FourStarType::Loss(FourStarLoss::Character))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    FourStarLoss::LightCone => {
                        self.light_cone_pulls += 1;
                        pity_entry
                            .entry(FourStarType::Loss(FourStarLoss::LightCone))
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }
        self.pity = 0;
    }
}
