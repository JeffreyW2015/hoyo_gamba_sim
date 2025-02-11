use std::time::Instant;

use hgs_core::{enums::FiveStarType, settings::Settings, GachaState};
use history::{FiveStarMetadata, PullHistory};
use rand::rng;

mod history;

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
    display_four_star_info(&pulls);

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
        println!(
            "expect a 4* every {} pulls",
            pulls.total_count() / metadata.total()
        )
    }
}
