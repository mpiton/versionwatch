use super::{Collector, Error};
use crate::GitHubTag;
use async_trait::async_trait;
use polars::prelude::*;
use semver::Version;

const GITHUB_API_URL: &str = "https://api.github.com/repos/python/cpython/tags";

pub struct PythonCollector {
    github_token: Option<String>,
}

impl PythonCollector {
    pub fn new(github_token: Option<String>) -> Self {
        Self { github_token }
    }
}

#[async_trait]
impl Collector for PythonCollector {
    fn name(&self) -> &'static str {
        "python"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let client = reqwest::Client::new();
        let mut request = client
            .get(GITHUB_API_URL)
            .header("User-Agent", "versionwatch-collector")
            .header("Accept", "application/vnd.github.v3+json");

        if let Some(token) = &self.github_token {
            request = request.bearer_auth(token);
        }

        let tags: Vec<GitHubTag> = request.send().await?.json().await?;

        let latest_version = tags
            .iter()
            .filter_map(|tag| Version::parse(tag.name.trim_start_matches('v')).ok())
            .filter(|v| v.pre.is_empty()) // Filter out pre-releases (alpha, beta, rc)
            .max()
            .ok_or(Error::NotFound)?;

        let df = df!(
            "name" => &["python"],
            "current_version" => &[""],
            "latest_version" => &[latest_version.to_string()],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[None::<String>],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
