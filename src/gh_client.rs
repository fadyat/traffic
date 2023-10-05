use std::fmt::{Display, Formatter};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::error::Error;

pub struct GitHubClient {
    client: Client,
    token: String,
}

#[derive(Deserialize)]
pub struct RepoViewsResponse {
    pub count: u64,
    pub uniques: u64,
    pub views: Vec<RepoView>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RepoView {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

impl Display for RepoView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", serde_json::to_string(self).unwrap());
    }
}


impl GitHubClient {
    const BASE_URL: &'static str = "https://api.github.com";
    const USER_AGENT: &'static str = "traffic-viewer";

    pub fn new(token: String) -> Self {
        let client = reqwest::Client::new();
        return GitHubClient { client, token };
    }

    pub async fn get_repo_views(
        self,
        owner: String,
        repo: String,
    ) -> Result<RepoViewsResponse, Error> {
        let url = format!("{}/repos/{}/{}/traffic/views", GitHubClient::BASE_URL, owner, repo);
        let resp_res = self.client.get(url)
            .bearer_auth(self.token)
            .header("User-Agent", GitHubClient::USER_AGENT)
            .send()
            .await;

        let resp = match resp_res {
            Ok(r) => r,
            Err(e) => return Err(Error { message: e.to_string() }),
        };

        if !resp.status().is_success() {
            return Err(Error { message: format!("HTTP error: {}", resp.status()) });
        }

        return match resp.json::<RepoViewsResponse>().await {
            Ok(r) => Ok(r),
            Err(e) => Err(Error { message: e.to_string() }),
        };
    }
}