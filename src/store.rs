use crate::api::github::RepoView;
use anyhow::{anyhow, Result};
use std::fs::File;

pub fn get_stored(path: &String) -> Result<Vec<RepoView>> {
    let storage_file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };

    match serde_json::from_reader(storage_file) {
        Ok(v) => Ok(v),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn save(path: &String, val: &Vec<RepoView>) -> Result<()> {
    let storage_file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(anyhow!(e)),
    };

    match serde_json::to_writer(storage_file, val) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}
