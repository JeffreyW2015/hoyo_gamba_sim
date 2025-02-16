// use std::{u32, u64};

// use hgs_core::{
//     enums::{self, FiveStarType, FourStarBanner, FourStarType, Pull},
//     settings::Settings,
//     GachaState,
// };

// mod mock_rng;

// use mock_rng::MockRng;

// fn run_pull_test(settings: &Settings, roll_sequence: Vec<u64>, expected_pull: Pull) {
//     let rng = &mut MockRng::new(roll_sequence);
//     let mut state = GachaState::new(settings.clone());

//     let pull = state.pull(rng);
//     assert_eq!(pull, expected_pull);
// }

// #[test]
// fn test_pull_three_star() {
//     let settings = Settings::from_banner(enums::Banner::Character);
//     let lower_bound =
//         ((settings.five_star_settings.base_rate + settings.four_star_settings.base_rate + 0.01)
//             * u64::MAX as f64) as u64;
//     let upper_bound = u64::MAX;

//     let test_cases = vec![vec![lower_bound], vec![upper_bound]];

//     for roll_sequence in test_cases {
//         run_pull_test(&settings, roll_sequence, Pull::ThreeStar);
//     }
// }

// #[test]
// fn test_pull_four_star() {
//     let settings = Settings::from_banner(enums::Banner::Character);

//     let lower_bound = ((settings.five_star_settings.base_rate + 0.001) * u64::MAX as f64) as u64;
//     let upper_bound = ((settings.five_star_settings.base_rate
//         + settings.four_star_settings.base_rate)
//         * u64::MAX as f64) as u64;

//     let roll_character = 0;
//     let roll_light_cone = u32::MAX as u64;

//     let roll_banner = 0;
//     let roll_non_banner = u64::MAX;

//     let roll_banner_a = vec![lower_bound, roll_character, roll_banner, 0];
//     let roll_banner_b = vec![
//         upper_bound,
//         roll_character,
//         roll_banner,
//         (u32::MAX / 2) as u64,
//     ];
//     let roll_banner_c = vec![lower_bound, roll_character, roll_banner, u32::MAX as u64];
//     let roll_non_banner_character = vec![lower_bound, roll_character, roll_non_banner];
//     let roll_light_cone_sequence = vec![lower_bound, roll_light_cone];

//     let test_cases = vec![
//         (
//             roll_banner_a,
//             Pull::FourStar(FourStarType::Banner(FourStarBanner::A)),
//         ),
//         (
//             roll_banner_b,
//             Pull::FourStar(FourStarType::Banner(FourStarBanner::B)),
//         ),
//         (
//             roll_banner_c,
//             Pull::FourStar(FourStarType::Banner(FourStarBanner::C)),
//         ),
//         (
//             roll_non_banner_character,
//             Pull::FourStar(FourStarType::Loss(enums::FourStarLoss::Character)),
//         ),
//         (
//             roll_light_cone_sequence,
//             Pull::FourStar(FourStarType::Loss(enums::FourStarLoss::LightCone)),
//         ),
//     ];

//     for (roll_sequence, expected_pull) in test_cases {
//         run_pull_test(&settings, roll_sequence, expected_pull);
//     }
// }

// #[test]
// fn test_pull_five_star() {
//     let settings = Settings::from_banner(enums::Banner::Character);
//     let lower_bound = 0;
//     let upper_bound = ((settings.five_star_settings.base_rate) * u64::MAX as f64) as u64;

//     let roll_limited = vec![lower_bound, 0];
//     let roll_standard = vec![upper_bound, u64::MAX];

//     let test_cases = vec![
//         (roll_limited, Pull::FiveStar(FiveStarType::Limited)),
//         (roll_standard, Pull::FiveStar(FiveStarType::Standard)),
//     ];

//     for (roll_sequence, expected_pull) in test_cases {
//         run_pull_test(&settings, roll_sequence, expected_pull);
//     }
// }
