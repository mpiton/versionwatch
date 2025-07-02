use super::{Collector, Error, GitHubRelease};
use async_trait::async_trait;
use polars::prelude::*;

pub struct NginxCollector {
    github_token: Option<String>,
}

impl NginxCollector {
    pub fn new(github_token: Option<String>) -> Self {
        Self { github_token }
    }
}

#[async_trait]
impl Collector for NginxCollector {
    fn name(&self) -> &'static str {
        "nginx"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let url = "https://api.github.com/repos/nginx/nginx/releases";
        let client = reqwest::Client::new();
        let mut request = client
            .get(url)
            .header("User-Agent", "versionwatch-collector")
            .header("Accept", "application/vnd.github.v3+json");
        if let Some(token) = &self.github_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;
        if response.status() == reqwest::StatusCode::FORBIDDEN
            || response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS
        {
            if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
                if remaining == "0" {
                    let reset = response
                        .headers()
                        .get("x-ratelimit-reset")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok());
                    let wait = reset.unwrap_or(0);
                    let wait_msg =
                        format!("GitHub API rate limit exceeded. Try again in {wait} seconds.");
                    return Err(Error::RateLimited(wait_msg));
                }
            }
            return Err(Error::Other(format!(
                "GitHub API returned forbidden or rate limited: {}",
                response.status()
            )));
        }
        let releases: Vec<GitHubRelease> = response.json().await?;

        let latest_release = releases
            .into_iter()
            .find(|r| !r.prerelease && !r.draft)
            .ok_or(Error::NotFound)?;

        // Extract version from tag_name, removing "release-" prefix
        let version = latest_release
            .tag_name
            .trim_start_matches("release-")
            .to_string();

        let df = df!(
            "name" => &["nginx"],
            "current_version" => &[None::<String>],
            "latest_version" => &[version],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some(latest_release.html_url)],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
