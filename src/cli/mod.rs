use crate::api::github;
use crate::config::Config;
use crate::store::{get_stored, save};
use log::info;

fn merge_views(old: Vec<github::RepoView>, new: Vec<github::RepoView>) -> Vec<github::RepoView> {
    if old.is_empty() {
        return new;
    }

    let mut merged = old.clone();
    let top_idx = merged
        .iter()
        .rposition(|view| view.timestamp == new[0].timestamp);

    if let Some(idx) = top_idx {
        merged.truncate(idx);
    }

    merged.extend_from_slice(&new);
    merged
}

#[tokio::main]
pub async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let c = Config::new(".config/config.yaml".to_string()).expect("failed to initialize config");

    let fetched = github::Client::new(c.github.token)
        .get_repo_views(&c.github.owner, &c.github.repo)
        .await
        .expect("failed to fetch repository traffic");

    let stored = get_stored(&c.storage.state_path).expect("failed to retrieve data from storage");

    let merged_views = merge_views(stored, fetched.views);
    info!("{} views fetched", merged_views.len());
    save(&c.storage.state_path, &merged_views).expect("failed to save the data");
    info!("new fetched data is merged with the old one");

    Ok(())
}
