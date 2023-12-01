use crate::api::github;
use crate::config::Config;
use crate::sync::sync;
use anyhow::Result;

pub async fn run_cli(c: &Config) -> Result<()> {
    let gh_client = github::Client::new(c.github.token.clone());

    sync(gh_client, &c.github.owner, &c.github.repo, &c.storage.state_path)
        .await
}
