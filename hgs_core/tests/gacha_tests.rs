use std::{u32, u64};

use hgs_core::{
    enums::{self, FiveStarType, FourStarBanner, FourStarType, Pull},
    settings::Settings,
    GachaState,
};

mod mock_rng;

use mock_rng::MockRng;

#[test]
fn test_pull_three_star() {
    let settings = Settings::from_banner(enums::Banner::Character);
    let lower_bound =
        ((settings.five_star_settings.base_rate + settings.four_star_settings.base_rate + 0.01)
            * u64::MAX as f64) as u64;
    let upper_bound = u64::MAX;

    let rng = &mut MockRng::new(vec![lower_bound, upper_bound]);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);

    assert_eq!(pull, Pull::ThreeStar);
}

#[test]
fn test_pull_four_star() {
    let settings = Settings::from_banner(enums::Banner::Character);
    let lower_bound = ((settings.five_star_settings.base_rate + 0.001) * u64::MAX as f64) as u64;
    let upper_bound = ((settings.five_star_settings.base_rate
        + settings.four_star_settings.base_rate)
        * u64::MAX as f64) as u64;
    let roll_character = 0;
    let roll_light_cone = u32::MAX as u64;
    let roll_banner = 0;
    let roll_non_banner = u64::MAX;

    // range 3,
    let roll_banner_a = 0;
    let roll_banner_b = (u32::MAX / 2) as u64;
    let roll_banner_c = u32::MAX as u64;

    let roll_a_sequence = vec![lower_bound, roll_character, roll_banner, roll_banner_a];
    let roll_b_sequence = vec![upper_bound, roll_character, roll_banner, roll_banner_b];
    let roll_c_sequence = vec![lower_bound, roll_character, roll_banner, roll_banner_c];
    let roll_non_banner_character_sequence =
        vec![lower_bound, roll_character, roll_non_banner, roll_banner_a];
    let roll_light_cone_sequence = vec![lower_bound, roll_light_cone];

    let rng: &mut MockRng = &mut MockRng::new(roll_a_sequence);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);
    assert_eq!(
        pull,
        Pull::FourStar(FourStarType::Banner(FourStarBanner::A))
    );

    let rng: &mut MockRng = &mut MockRng::new(roll_b_sequence);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);
    assert_eq!(
        pull,
        Pull::FourStar(FourStarType::Banner(FourStarBanner::B))
    );

    let rng: &mut MockRng = &mut MockRng::new(roll_c_sequence);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);
    assert_eq!(
        pull,
        Pull::FourStar(FourStarType::Banner(FourStarBanner::C))
    );

    let rng: &mut MockRng = &mut MockRng::new(roll_non_banner_character_sequence);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);
    assert_eq!(
        pull,
        Pull::FourStar(FourStarType::Loss(enums::FourStarLoss::Character))
    );

    let rng: &mut MockRng = &mut MockRng::new(roll_light_cone_sequence);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);
    assert_eq!(
        pull,
        Pull::FourStar(FourStarType::Loss(enums::FourStarLoss::LightCone))
    );
}

#[test]
fn test_pull_five_star() {
    let settings = Settings::from_banner(enums::Banner::Character);
    let lower_bound = 0;
    let roll_banner = 0;
    let roll_standard = u64::MAX;

    let roll_limited_sequence = vec![lower_bound, roll_banner];
    let roll_standard_sequence = vec![lower_bound, roll_standard];

    let rng: &mut MockRng = &mut MockRng::new(roll_limited_sequence);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);
    assert_eq!(pull, Pull::FiveStar(FiveStarType::Limited));

    let rng: &mut MockRng = &mut MockRng::new(roll_standard_sequence);
    let mut state = GachaState::new(settings);

    let pull = state.pull(rng);
    assert_eq!(pull, Pull::FiveStar(FiveStarType::Standard));
}