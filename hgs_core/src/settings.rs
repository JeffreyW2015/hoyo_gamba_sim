use std::{fs::File, io::Read};

use crate::enums::Banner;

#[derive(serde::Deserialize, Debug, Copy, Clone)]
pub struct Settings {
    pub five_star_settings: RaritySettings,
    pub four_star_settings: RaritySettings,
}

#[derive(serde::Deserialize, Debug, Copy, Clone)]
pub struct RaritySettings {
    pub base_rate: f64,
    pub hard_pity: Option<u64>,
    pub soft_pity_settings: Option<SoftPitySettings>,
    pub banner_rate: Option<f64>,

    pub guarantee_enabled: bool,
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct SoftPitySettings {
    pub pity_start: u64,
    pub rate_increase: f64,
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
                five_star_settings: RaritySettings {
                    base_rate: 0.006,
                    hard_pity: Some(90),
                    soft_pity_settings: Some(SoftPitySettings {
                        pity_start: 74,
                        rate_increase: 0.06,
                    }),
                    banner_rate: Some(0.55),
                    guarantee_enabled: true,
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.051,
                    hard_pity: Some(10),
                    soft_pity_settings: None,
                    banner_rate: Some(0.5),
                    guarantee_enabled: true,
                },
            },
            Banner::LightCone => Settings {
                five_star_settings: RaritySettings {
                    base_rate: 0.008,
                    hard_pity: Some(80),
                    soft_pity_settings: Some(SoftPitySettings {
                        pity_start: 66,
                        rate_increase: 0.07,
                    }),
                    banner_rate: Some(0.75),
                    guarantee_enabled: true,
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.066,
                    hard_pity: Some(10),
                    soft_pity_settings: None,
                    banner_rate: Some(0.75),
                    guarantee_enabled: true,
                },
            },
            Banner::Standard => Settings {
                five_star_settings: RaritySettings {
                    base_rate: 0.006,
                    hard_pity: Some(90),
                    soft_pity_settings: Some(SoftPitySettings {
                        pity_start: 74,
                        rate_increase: 0.06,
                    }),
                    banner_rate: None,
                    guarantee_enabled: false,
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.051,
                    hard_pity: Some(10),
                    soft_pity_settings: None,
                    banner_rate: None,
                    guarantee_enabled: false,
                },
            },
            Banner::Departure => Settings {
                five_star_settings: RaritySettings {
                    base_rate: 0.006,
                    hard_pity: Some(50),
                    soft_pity_settings: None,
                    banner_rate: None,
                    guarantee_enabled: false,
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.051,
                    hard_pity: Some(10),
                    soft_pity_settings: None,
                    banner_rate: None,
                    guarantee_enabled: false,
                },
            },
        }
    }
}
