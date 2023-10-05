use std::fs::File;
use crate::error::Error;
use crate::gh_client::RepoView;

pub fn get_stored(path: String) -> Result<Vec<RepoView>, Error> {
    let storage_file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new())
    };

    return match serde_json::from_reader(storage_file) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error { message: e.to_string() }),
    };
}

pub fn merge_and_store(
    path: String,
    old_views: Vec<RepoView>,
    new_views: Vec<RepoView>,
) -> Result<(), Error> {
    if old_views.is_empty() {
        return save(path, new_views);
    }

    let mut merged_views = old_views.clone();
    let top_idx = merged_views.iter()
        .rposition(|view| view.timestamp == new_views[0].timestamp);

    if let Some(idx) = top_idx {
        merged_views.truncate(idx);
    }

    merged_views.extend_from_slice(&new_views);
    return save(path, merged_views);
}

fn save(path: String, mut val: Vec<RepoView>) -> Result<(), Error> {
    let storage_file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(Error { message: e.to_string() }),
    };

    val.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    return match serde_json::to_writer(storage_file, &val) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error { message: e.to_string() }),
    };
}