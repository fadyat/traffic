use crate::api::github;
use crate::merger::merge_views;
use crate::store::{get_stored, save};
use anyhow::{anyhow, Result};

pub async fn sync(
    gh: github::Client,
    owner: &String,
    repo: &String,
    state_path: &String,
) -> Result<()> {
    let fetched = match gh.get_repo_views(owner, repo).await {
        Ok(f) => f,
        Err(e) => return Err(anyhow!(e)),
    };

    let stored = match get_stored(state_path) {
        Ok(s) => s,
        Err(e) => return Err(anyhow!(e)),
    };

    let merged_views = merge_views(stored, fetched.views);
    save(state_path, &merged_views)
}
