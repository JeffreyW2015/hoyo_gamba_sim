use std::{collections::HashMap, time::Instant};

use hgs_core::{
    enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Rarity},
    settings::Settings,
    GachaState,
};
use rand::rng;

struct FiveStarMetadata {
    pity_to_pulls: HashMap<u8, HashMap<FiveStarType, usize>>,
    limited_pulls: usize,
    standard_pulls: usize,
    pity: u8,

    next_guaranteed: bool,
    wins: usize,
    losses: usize,
    guaranteeds: usize,
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

    fn total(&self) -> usize {
        self.limited_pulls + self.standard_pulls
    }

    fn _pity_of_type(&self, pity: u8, five_star: FiveStarType) -> usize {
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

struct FourStarMetadata {
    pity_to_pulls: HashMap<u8, HashMap<FourStarType, usize>>,
    a_pulls: usize,
    b_pulls: usize,
    c_pulls: usize,
    character_non_banner_pulls: usize,
    light_cone_pulls: usize,
    pity: u8,

    next_guaranteed: bool,
    wins: usize,
    losses: usize,
    guaranteeds: usize,
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

    fn banners(&self) -> usize {
        self.a_pulls + self.b_pulls + self.c_pulls
    }

    fn non_banners(&self) -> usize {
        self.character_non_banner_pulls + self.light_cone_pulls
    }

    fn total(&self) -> usize {
        self.banners() + self.non_banners()
    }

    fn _pity_of_type(&self, pity: u8, four_star: FourStarType) -> usize {
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

struct PullHistory {
    pulls: Vec<Rarity>,
    five_star_metadata: FiveStarMetadata,
    four_star_metadata: FourStarMetadata,
}

impl PullHistory {
    fn new() -> Self {
        PullHistory {
            pulls: Vec::new(),
            five_star_metadata: FiveStarMetadata::new(),
            four_star_metadata: FourStarMetadata::new(),
        }
    }

    fn add(&mut self, rarity: Rarity) {
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

    fn total_count(&self) -> usize {
        self.pulls.len()
    }

    fn five_star_metadata(&self) -> &FiveStarMetadata {
        &self.five_star_metadata
    }

    fn four_star_metadata(&self) -> &FourStarMetadata {
        &self.four_star_metadata
    }
}

fn main() {
    let start = Instant::now();
    let settings = Settings::load_from_file("settings.json").unwrap();
    let mut state = GachaState::new(settings);
    let mut rng: rand::prelude::ThreadRng = rng();

    let num_pulls = 1_000_000;
    let mut pulls = PullHistory::new();

    for _ in 0..num_pulls {
        pulls.add(state.pull(&mut rng))
    }

    println!("\nSummary");
    println!("=======\n");
    println!("Total pulls: {}", pulls.total_count());
    display_five_star_info(&pulls);
    // display_four_star_info(&pulls);

    let duration = start.elapsed();
    println!("Time taken {:.2?} seconds", duration);
}

fn _display_five_star_pity_map(metadata: &FiveStarMetadata) {
    let mut sorted_history: Vec<_> = metadata.pity_to_pulls.iter().collect();
    sorted_history.sort_by(|a, b| a.0.cmp(&b.0));
    for (pity_count, _) in sorted_history {
        println!(
            "Pity {}: {} total, {} Limited, {} Standard",
            pity_count,
            metadata._pity_of_type(*pity_count, FiveStarType::Limited)
                + metadata._pity_of_type(*pity_count, FiveStarType::Standard),
            metadata._pity_of_type(*pity_count, FiveStarType::Limited),
            metadata._pity_of_type(*pity_count, FiveStarType::Standard),
        );
    }
}

fn display_five_star_info(pulls: &PullHistory) {
    println!("\nFive Stars");
    println!("----------\n");
    let metadata = pulls.five_star_metadata();
    println!("Total Five Stars: {}", metadata.total());
    println!("Total Limited: {}", metadata.limited_pulls);
    if metadata.total() > 0 {
        println!(
            "Wins: {}, Losses: {}, Gauranteeds: {}, Win Ratio: {}",
            metadata.wins,
            metadata.losses,
            metadata.guaranteeds,
            (metadata.wins as f64) / ((metadata.wins + metadata.losses) as f64)
        );
    }
    println!("Total Standard: {}", metadata.standard_pulls);

    if metadata.total() > 0 {
        println!(
            "you can expect a 5* every {} pulls",
            pulls.total_count() / metadata.total()
        );
    }

    if metadata.limited_pulls > 0 {
        println!(
            "you can expect a limted every {} pulls",
            pulls.total_count() / metadata.limited_pulls,
        );
    }

    if metadata.total() > 0 {
        println!(
            "limited vs standard average rate: {}%",
            (metadata.limited_pulls as f64 / metadata.total() as f64) * 100f64
        );
    }
}

fn display_four_star_info(pulls: &PullHistory) {
    println!("\nFour Stars");
    println!("----------\n");
    let metadata = pulls.four_star_metadata();
    println!("Total: {}", metadata.total());
    println!(
        "A: {}, B: {}, C: {}, Total: {}",
        metadata.a_pulls,
        metadata.b_pulls,
        metadata.c_pulls,
        metadata.banners()
    );

    if metadata.banners() > 0 {
        let a_avg = {
            if metadata.a_pulls > 0 {
                pulls.total_count() / metadata.a_pulls
            } else {
                0
            }
        };
        let b_avg = {
            if metadata.b_pulls > 0 {
                pulls.total_count() / metadata.b_pulls
            } else {
                0
            }
        };
        let c_avg = {
            if metadata.c_pulls > 0 {
                pulls.total_count() / metadata.c_pulls
            } else {
                0
            }
        };
        println!(
            "N Pulls for\n  - A: {}\n  - B: {}\n  - C: {}\n  - Average for specific: {}\n  - Average for any banner: {}",
            a_avg,
            b_avg,
            c_avg,
            (a_avg + b_avg + c_avg) / 3,
            pulls.total_count() / metadata.banners()
        )
    }
    if metadata.total() > 0 {
        println!("expect a 4* every {} pulls", pulls.total_count() / metadata.total())
    }
}
