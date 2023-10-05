use std::fs::File;
use serde::Deserialize;
use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: GithubConfig,
    pub storage: StorageConfig,
}

impl Config {
    pub fn new(config_path: String) -> Result<Config, Error> {
        let config_file = match File::open(config_path) {
            Ok(f) => f,
            Err(e) => return Err(Error { message: e.to_string() }),
        };

        return match serde_yaml::from_reader(config_file) {
            Ok(c) => Ok(c),
            Err(e) => Err(Error { message: e.to_string() }),
        };
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
    pub plot_path: String,
}