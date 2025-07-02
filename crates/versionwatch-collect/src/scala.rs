use super::{Collector, Error, GitHubRelease};
use async_trait::async_trait;
use polars::prelude::*;

pub struct ScalaCollector {
    github_token: Option<String>,
}

impl ScalaCollector {
    pub fn new(github_token: Option<String>) -> Self {
        Self { github_token }
    }
}

#[async_trait]
impl Collector for ScalaCollector {
    fn name(&self) -> &'static str {
        "scala"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let url = "https://api.github.com/repos/scala/scala/releases";
        let client = reqwest::Client::new();
        let mut request = client
            .get(url)
            .header("User-Agent", "versionwatch-collector");
        if let Some(token) = &self.github_token {
            request = request.bearer_auth(token);
        }
        let releases: Vec<GitHubRelease> = request.send().await?.json().await?;

        let latest_release = releases
            .into_iter()
            .find(|r| !r.prerelease && !r.draft)
            .ok_or(Error::NotFound)?;

        let df = df!(
            "name" => &["scala"],
            "current_version" => &[""],
            "latest_version" => &[latest_release.tag_name],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some(latest_release.html_url)],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
