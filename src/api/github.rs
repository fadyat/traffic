use anyhow::{anyhow, Result};
use reqwest::{header, Client as HTTPClient};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub struct Client {
    client: HTTPClient,
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
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Client {
    const BASE_URL: &'static str = "https://api.github.com";
    const USER_AGENT: &'static str = "traffic-viewer";

    pub fn new(token: String) -> Self {
        Client {
            client: HTTPClient::new(),
            token,
        }
    }

    pub async fn get_repo_views(self, owner: &String, repo: &String) -> Result<RepoViewsResponse> {
        let url = format!(
            "{}/repos/{}/{}/traffic/views",
            Client::BASE_URL,
            owner,
            repo
        );
        let resp_res = self
            .client
            .get(url)
            .bearer_auth(self.token)
            .header(header::USER_AGENT, Client::USER_AGENT)
            .send()
            .await;

        let resp = match resp_res {
            Ok(r) => r,
            Err(e) => return Err(anyhow!(e)),
        };

        if !resp.status().is_success() {
            return Err(anyhow!(resp.text().await.unwrap()));
        }

        match resp.json::<RepoViewsResponse>().await {
            Ok(r) => Ok(r),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
