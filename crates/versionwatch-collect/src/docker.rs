use super::{Collector, Error, GitHubRelease};
use async_trait::async_trait;
use polars::prelude::*;

pub struct DockerCollector {
    github_token: Option<String>,
}

impl DockerCollector {
    pub fn new(github_token: Option<String>) -> Self {
        Self { github_token }
    }
}

#[async_trait]
impl Collector for DockerCollector {
    fn name(&self) -> &'static str {
        "docker"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let url = "https://api.github.com/repos/moby/moby/releases";
        let client = reqwest::Client::new();
        let mut request = client
            .get(url)
            .header("User-Agent", "versionwatch-collector")
            .header("Accept", "application/vnd.github.v3+json");
        if let Some(token) = &self.github_token {
            request = request.bearer_auth(token);
        }
        let releases: Vec<GitHubRelease> = request.send().await?.json().await?;

        let latest_release = releases
            .into_iter()
            .find(|r| !r.prerelease && !r.draft)
            .ok_or(Error::NotFound)?;

        let version = latest_release.tag_name.trim_start_matches("v").to_string();

        let df = df!(
            "name" => &["docker"],
            "current_version" => &[""],
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
