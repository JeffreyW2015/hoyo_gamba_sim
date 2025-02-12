use std::{fs::File, io::Read};

use crate::enums::Banner;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub five_star_settings: RaritySettings,
    pub four_star_settings: RaritySettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct RaritySettings {
    pub base_rate: f64,
    pub hard_pity: u8,
    pub soft_pity_settings: Option<SoftPitySettings>,
    pub limited_rate: Option<f64>,
}

impl RaritySettings {
    pub fn guarantee(&self) -> Option<bool> {
        self.limited_rate.map(|_| false)
    }
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct SoftPitySettings {
    pub pity_start: u8,
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
                    hard_pity: 90,
                    soft_pity_settings: Some(SoftPitySettings {
                        pity_start: 74,
                        rate_increase: 0.06
                    }),
                    limited_rate: Some(0.55),
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.051,
                    hard_pity: 10,
                    soft_pity_settings: None,
                    limited_rate: Some(0.5),
                },
            },
            Banner::LightCone => Settings {
                five_star_settings: RaritySettings {
                    base_rate: 0.008,
                    hard_pity: 80,
                    soft_pity_settings: Some(SoftPitySettings {
                        pity_start: 66,
                        rate_increase: 0.07
                    }),
                    limited_rate: Some(0.75),
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.066, 
                    hard_pity: 10,
                    soft_pity_settings: None,
                    limited_rate: Some(0.75), 
                },
            },
            Banner::Standard => Settings {
                five_star_settings: RaritySettings {
                    base_rate: 0.006, 
                    hard_pity: 90,
                    soft_pity_settings: Some(SoftPitySettings {
                        pity_start: 74,
                        rate_increase: 0.06
                    }),
                    limited_rate: None,
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.051,
                    hard_pity: 10,
                    soft_pity_settings: None,
                    limited_rate: None,
                },
            },
            Banner::Departure => Settings {
                five_star_settings: RaritySettings {
                    base_rate: 0.006, 
                    hard_pity: 50,
                    soft_pity_settings: None,
                    limited_rate: None,
                },
                four_star_settings: RaritySettings {
                    base_rate: 0.051,
                    hard_pity: 10,
                    soft_pity_settings: None,
                    limited_rate: None,
                },
            },
        }
    }
}
