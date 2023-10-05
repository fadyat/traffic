use crate::config::Config;
use crate::gh_client::{GitHubClient, RepoView};
use crate::store::{get_stored, save};

mod gh_client;
mod config;
mod error;
mod store;
mod plot;

fn merge_views(old: Vec<RepoView>, new: Vec<RepoView>) -> Vec<RepoView> {
    if old.is_empty() {
        return new;
    }

    let mut merged = old.clone();
    let top_idx = merged.iter()
        .rposition(|view| view.timestamp == new[0].timestamp);

    if let Some(idx) = top_idx {
        merged.truncate(idx);
    }

    merged.extend_from_slice(&new);
    return merged;
}

#[tokio::main]
async fn main() {
    let c = Config::new(".config/config.yaml".to_string())
        .expect("failed to initialize config");

    let fetched = GitHubClient::new(c.github.token)
        .get_repo_views(&c.github.owner, &c.github.repo)
        .await
        .expect("failed to fetch repository traffic");

    let stored = get_stored(&c.storage.state_path)
        .expect("failed to retrieve data from storage");

    let merged_views = merge_views(stored, fetched.views);
    save(&c.storage.state_path, &merged_views)
        .expect("failed to save the data");

    plot::update(c.storage.plot_path, &merged_views)
}
