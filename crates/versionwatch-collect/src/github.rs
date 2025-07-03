use super::{Collector, Error, GitHubRelease, GitHubTag};
use async_trait::async_trait;
use polars::prelude::*;
use semver::Version;

/// A collector for software that publishes releases on GitHub.
pub struct GitHubCollector {
    name: String,
    repo: String,
    github_token: Option<String>,
    source: String,
}

impl GitHubCollector {
    pub fn new(name: String, repo: String, github_token: Option<String>, source: String) -> Self {
        Self {
            name,
            repo,
            github_token,
            source,
        }
    }
}

#[async_trait]
impl Collector for GitHubCollector {
    fn name(&self) -> &'static str {
        // This is a bit of a hack to satisfy the trait bound.
        // The name is dynamic, so we leak it to get a 'static lifetime.
        Box::leak(self.name.clone().into_boxed_str())
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        if self.source == "releases" {
            self.collect_from_releases().await
        } else if self.source == "tags" {
            self.collect_from_tags().await
        } else {
            Err(Error::Other(format!(
                "Unsupported GitHub source: {}",
                self.source
            )))
        }
    }
}

impl GitHubCollector {
    async fn collect_from_releases(&self) -> Result<DataFrame, Error> {
        let url = format!("https://api.github.com/repos/{}/releases", self.repo);
        let releases: Vec<GitHubRelease> = self.fetch(&url).await?;

        let latest_release = releases
            .into_iter()
            .find(|r| !r.prerelease && !r.draft)
            .ok_or(Error::NotFound)?;

        let version = latest_release.tag_name.trim_start_matches('v').to_string();

        let df = df!(
            "name" => &[self.name.as_str()],
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

    async fn collect_from_tags(&self) -> Result<DataFrame, Error> {
        let url = format!("https://api.github.com/repos/{}/tags", self.repo);
        let tags: Vec<GitHubTag> = self.fetch(&url).await?;

        let latest_tag = tags
            .iter()
            .filter_map(|tag| {
                Version::parse(tag.name.trim_start_matches('v'))
                    .ok()
                    .filter(|v| v.pre.is_empty())
                    .map(|v| (v, tag))
            })
            .max_by(|(v1, _), (v2, _)| v1.cmp(v2))
            .map(|(_, tag)| tag)
            .ok_or(Error::NotFound)?;

        let version_string = latest_tag.name.trim_start_matches('v').to_string();

        let release_notes_url = format!(
            "https://github.com/{}/releases/tag/{}",
            self.repo, latest_tag.name
        );

        let df = df!(
            "name" => &[self.name.as_str()],
            "current_version" => &[""],
            "latest_version" => &[version_string],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some(release_notes_url)],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }

    async fn fetch<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
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

        let data: T = response.json().await?;
        Ok(data)
    }
}
