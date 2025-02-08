use hgs_core::{enums::{FiveStarType, Rarity}, settings::Settings, GachaState};
use rand::rng;
use std::collections::HashMap;

fn main() {
    let settings = Settings::load_from_file("settings.json").unwrap();
    let mut state = GachaState::new(settings);
    let mut rng: rand::prelude::ThreadRng = rng();
    let num_pulls = 180;

    let mut history: HashMap<u8, (u64, u64)> = HashMap::new();

    let mut previous_five_star: Option<FiveStarType> = None;
    let mut wins = 0;
    let mut guaranteeds = 0;
    let mut losses = 0;

    for _ in 0..num_pulls {
        let pity = state.pity + 1;
        let rarity = state.simulate_pull(&mut rng);
        if rarity == Rarity::FiveStar(FiveStarType::Standard) {
            let pity_entry = history.entry(pity).or_insert((0, 0));
            pity_entry.0 += 1;

            if previous_five_star == None || previous_five_star == Some(FiveStarType::Limited) {
                losses += 1;
            }
            previous_five_star = Some(FiveStarType::Standard);
        } else if rarity == Rarity::FiveStar(FiveStarType::Limited) {
            let pity_entry = history.entry(pity).or_insert((0, 0));
            pity_entry.0 += 1;
            pity_entry.1 += 1;

            if previous_five_star == None || previous_five_star == Some(FiveStarType::Limited) {
                wins += 1;
            }
            if previous_five_star == Some(FiveStarType::Standard) {
                guaranteeds += 1;
            }
            previous_five_star = Some(FiveStarType::Limited);
        }
    }

    let mut sorted_history: Vec<_> = history.iter().collect();
    sorted_history.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total_five_stars = 0;
    let mut total_limited = 0;

    for (pity_count, (total, limited)) in sorted_history {
        println!(
            "Pity {}: {} total, {} Limited, {} Standard",
            pity_count,
            total,
            limited,
            total - limited
        );
        total_five_stars += total;
        total_limited += limited;
    }

    println!("\nSummary");
    println!("Total Five Stars: {}", total_five_stars);
    println!("Total Limited: {}", total_limited);
    if total_five_stars > 0 {
        println!(
            "Wins: {}, Losses: {}, Gauranteeds: {}, Win Ratio: {}",
            wins,
            losses,
            guaranteeds,
            (wins as f64) / ((wins + losses) as f64)
        );
    }
    println!("Total Standard: {}", total_five_stars - total_limited);

    if total_five_stars > 0 {
        println!(
            "you can expect a 5* every {} pulls",
            num_pulls / total_five_stars
        );
    }

    if total_limited > 0 {
        println!(
            "you can expect a limted every {} pulls",
            num_pulls / total_limited
        );
    }

    if total_five_stars > 0 {
        println!(
            "limited vs standard average rate: {}%",
            (total_limited as f64 / total_five_stars as f64) * 100f64
        );
    }
}
