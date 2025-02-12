use std::{fs::File, io::Read};

use crate::enums::Banner;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub five_star_base_rate: f64,
    pub five_star_hard_pity: u8,
    pub five_star_soft_pity: u8,
    pub five_star_soft_pity_rate_increase: f64,
    pub five_star_limited_rate: f64,

    pub four_star_base_rate: f64,
    pub four_star_hard_pity: u8,
    pub four_star_banner_rate: f64,
}

impl Settings {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let settings: Settings = serde_json::from_str(&contents)?;
        Ok(settings)
    }

    pub fn from_banner(banner: Banner) -> Self {
        match banner {
            Banner::Character => Settings {
                five_star_base_rate: 0.006, // 0.6%
                five_star_hard_pity: 90,
                five_star_soft_pity: 74,
                five_star_soft_pity_rate_increase: 0.06, // 6%
                five_star_limited_rate: 0.55, // 50/50 is more like 55/45, maybe even higher

                four_star_base_rate: 0.051, // 5.1%
                four_star_hard_pity: 10,
                four_star_banner_rate: 0.5, // 50%
            },
            Banner::LightCone => Settings {
                five_star_base_rate: 0.008, // 0.8%
                five_star_hard_pity: 80,
                five_star_soft_pity: 66,
                five_star_soft_pity_rate_increase: 0.07, // 7%
                five_star_limited_rate: 0.75, // 75%

                four_star_base_rate: 0.066, // 6.6%
                four_star_hard_pity: 10,
                four_star_banner_rate: 0.75, // 75%
            },
            Banner::Standard => Settings {
                five_star_base_rate: 0.006, // 0.6%
                five_star_hard_pity: 90,
                five_star_soft_pity: 74,
                five_star_soft_pity_rate_increase: 0.06, // 6%
                five_star_limited_rate: 0.55, // 50/50 is more like 55/45, maybe even higher

                four_star_base_rate: 0.051, // 5.1%
                four_star_hard_pity: 10,
                four_star_banner_rate: 0.5, // 50%
            },
            Banner::Departure => Settings {
                five_star_base_rate: 0.006, // 0.6%
                five_star_hard_pity: 90,
                five_star_soft_pity: 74,
                five_star_soft_pity_rate_increase: 0.06, // 6%
                five_star_limited_rate: 0.55, // 50/50 is more like 55/45, maybe even higher

                four_star_base_rate: 0.051, // 5.1%
                four_star_hard_pity: 10,
                four_star_banner_rate: 0.5, // 50%
            },
        }
    }
}
