use std::{fs::File, io::Read};

use crate::enums::Pull;

pub enum SettingsError {
    InvalidSettings
}

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub pools: Vec<Pool>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Pool {
    pub name: String,
    pub rate: f64, // Rates for any collection of pools must sum <= 1.0
    pub contents: PoolContents,

    pub soft_pity_settings: Option<SoftPitySettings>,
    pub hard_pity: Option<u64>,
    pub guaranteed_pool: Option<PoolRef>
}

#[derive(serde::Deserialize, Debug)]
pub enum PoolContents {
    Pulls(Vec<Pull>),
    SubPools(Vec<Pool>),
}

#[derive(serde::Deserialize, Debug)]
pub struct SoftPitySettings {
    pub pity_start: u64,
    pub rate_increase: f64,
}

#[derive(serde::Deserialize, Debug)]
pub enum PoolRef {
    ByName(String),
}

impl Settings {
    pub fn new(pools: Vec<Pool>) -> Result<Self, SettingsError> {
        let settings = Settings {pools};
        if settings.is_valid(){
            Ok(settings)
        } else {
            Err(SettingsError::InvalidSettings)
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let settings: Settings = serde_json::from_str(&contents)?;
        Ok(settings)
    }

    pub fn is_valid(&self) -> bool {
        true // temporary until fleshed out
    }
}
