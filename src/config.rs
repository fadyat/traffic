use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: GithubConfig,
    pub storage: StorageConfig,
}

impl Config {
    pub fn new(config_path: String) -> Result<Config> {
        let config_file = match File::open(config_path) {
            Ok(f) => f,
            Err(e) => return Err(anyhow!(e)),
        };

        match serde_yaml::from_reader(config_file) {
            Ok(c) => Ok(c),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GithubConfig {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub state_path: String,
}
