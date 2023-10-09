use std::fs::File;
use crate::error::Error;
use crate::gh_client::RepoView;

pub fn get_stored(path: &String) -> Result<Vec<RepoView>, Error> {
    let storage_file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new())
    };

    match serde_json::from_reader(storage_file) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error { message: e.to_string() }),
    }
}

pub fn save(path: &String, val: &Vec<RepoView>) -> Result<(), Error> {
    let storage_file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(Error { message: e.to_string() }),
    };

    match serde_json::to_writer(storage_file, val) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error { message: e.to_string() }),
    }
}