use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: GithubConfig,
    pub storage: StorageConfig,
    #[serde(default)]
    pub app: AppConfig,
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

#[derive(Debug, Deserialize)]
pub struct AppConfig {

    #[serde(default = "default_window_size")]
    pub window_size: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            window_size: default_window_size(),
        }
    }
}

fn default_window_size() -> usize { 30 }
