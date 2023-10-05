use crate::config::Config;
use crate::gh_client::GitHubClient;
use crate::store::{get_stored, merge_and_store};

mod gh_client;
mod config;
mod error;
mod store;

#[tokio::main]
async fn main() {
    let c = Config::new(".config/config.yaml".to_string())
        .expect("failed to initialize config");

    let new_views = GitHubClient::new(c.github.token)
        .get_repo_views(c.github.owner, c.github.repo)
        .await
        .expect("failed to fetch repository traffic");

    let stored_views = get_stored(c.storage.path.clone())
        .expect("failed to retrieve data from storage");

    merge_and_store(c.storage.path.clone(), stored_views, new_views.views)
        .expect("failed to merge and store the data");
}
